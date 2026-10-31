use crate::prelude::*;

use crate::web::upload::{push_percent_encoded, sanitize_file_name};
use crate::web::{Upload, UploadError, http};

use axum::body::Body;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use std::io::ErrorKind;

// El navegador puede guardar el archivo, pero debe revalidarlo antes de reutilizarlo, así que el
// handler vuelve a comprobar los permisos; las cachés compartidas no lo guardan.
const DOWNLOAD_CACHE_CONTROL: http::HeaderValue =
    http::HeaderValue::from_static("private, no-cache");

// Cabeceras de la petición que usa `ServeFile` para responder a peticiones parciales y
// condicionales; el resto (cookies incluidas) no le hacen falta.
const FORWARDED_HEADERS: [http::HeaderName; 6] = [
    http::header::RANGE,
    http::header::IF_RANGE,
    http::header::IF_MATCH,
    http::header::IF_NONE_MATCH,
    http::header::IF_MODIFIED_SINCE,
    http::header::IF_UNMODIFIED_SINCE,
];

/// Sirve un archivo subido con [`Upload`] como respuesta a una petición.
///
/// Es la forma de publicar los archivos subidos **con control de acceso**: el handler comprueba
/// los permisos y busca el archivo (normalmente en la base de datos), y `Download` lo envía con las
/// cabeceras adecuadas:
///
/// - `Content-Type` según la extensión del archivo guardado, que fija el servidor y no el usuario.
/// - `Content-Disposition` con el nombre de [`with_file_name()`](Self::with_file_name),
///   normalmente el original. Las imágenes (salvo SVG), el audio, el vídeo, los PDF y el texto
///   plano se muestran en el navegador (`inline`); el resto se descarga (`attachment`), para que
///   un `.html` o un `.svg` subido no se ejecute como una página más del sitio.
///   [`with_attachment()`](Self::with_attachment) obliga a descargar también los primeros.
/// - `X-Content-Type-Options: nosniff`, para que el navegador no deduzca otro tipo del contenido.
/// - `Cache-Control: private, no-cache`: el navegador revalida el archivo antes de reutilizarlo,
///   así que cada uso vuelve a pasar por el handler, y las cachés compartidas no lo guardan.
///
/// Atiende también las peticiones parciales (`Range`, para reanudar una descarga o avanzar en un
/// vídeo) y condicionales (`If-None-Match` con el `ETag` de la respuesta, o `If-Modified-Since`),
/// con `206 Partial Content` o `304 Not Modified`.
///
/// Sólo sirve archivos del directorio de archivos subidos. La ruta debe salir de la aplicación (la
/// guardada en la base de datos), nunca directamente de la petición: aunque no puede salir de ese
/// directorio, daría acceso a cualquier archivo subido.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// async fn invoice(
///     request: HttpRequest,
///     web::Path(id): web::Path<u32>,
/// ) -> Result<Response, ErrorPage> {
///     // Comprobar los permisos y buscar en la base de datos el archivo de la factura `id`...
///     # let (path, original_name) = (id.to_string(), "invoice.pdf");
///     web::Download::new(path)
///         .with_file_name(original_name)
///         .serve(&request)
///         .await
///         .map_err(|err| match err {
///             web::UploadError::NotFound(_) => ErrorPage::NotFound(Some(request)),
///             _ => ErrorPage::InternalError(Some(request)),
///         })
/// }
/// ```
#[derive(Clone, Debug, Getters)]
pub struct Download {
    /// Devuelve la ruta del archivo, relativa al directorio de archivos subidos
    /// ([`UploadedFile::path()`](crate::web::UploadedFile::path)).
    path: String,
    /// Devuelve el nombre con el que se muestra o se guarda el archivo. Vacío, se usa el del
    /// archivo guardado.
    file_name: String,
    /// Devuelve si el archivo se descarga siempre, aunque el navegador pudiera mostrarlo.
    #[getters(copy)]
    attachment: bool,
}

impl Download {
    /// Prepara la respuesta para el archivo guardado en `path`, relativa al directorio de archivos
    /// subidos ([`UploadedFile::path()`](crate::web::UploadedFile::path)).
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            file_name: String::new(),
            attachment: false,
        }
    }

    /// Construye la respuesta con el archivo, atendiendo a las cabeceras de `request`.
    ///
    /// Devuelve [`UploadError::NotFound`] si el archivo no existe, para que el handler responda
    /// con [`ErrorPage::NotFound`](crate::response::ErrorPage::NotFound), e
    /// [`UploadError::InvalidPath`] si la ruta sale del directorio de archivos subidos.
    pub async fn serve(&self, request: &HttpRequest) -> Result<Response, UploadError> {
        let local = Upload::local_path(self.path())?;
        match tokio::fs::metadata(&local).await {
            Ok(metadata) if metadata.is_file() => {}
            Ok(_) => return Err(UploadError::NotFound(self.path.clone())),
            Err(err) if err.kind() == ErrorKind::NotFound => {
                return Err(UploadError::NotFound(self.path.clone()));
            }
            Err(err) => return Err(err.into()),
        }

        // `ServeFile` deduce el tipo MIME de la extensión y resuelve las peticiones parciales y
        // condicionales con las cabeceras de la petición original.
        let mut file_request = http::Request::new(Body::empty());
        let headers = file_request.headers_mut();
        for name in FORWARDED_HEADERS {
            if let Some(value) = request.headers().get(&name) {
                headers.insert(name, value.clone());
            }
        }
        let Ok(response) = ServeFile::new(&local).oneshot(file_request).await;
        let mut response = response.map(Body::new);

        let inline = !self.attachment()
            && response
                .headers()
                .get(http::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .is_some_and(shows_inline);
        let file_name = if self.file_name().is_empty() {
            self.path().rsplit('/').next().unwrap_or_default()
        } else {
            self.file_name()
        };

        let headers = response.headers_mut();
        if let Ok(value) = http::HeaderValue::from_str(&content_disposition(inline, file_name)) {
            headers.insert(http::header::CONTENT_DISPOSITION, value);
        }
        headers.insert(
            http::header::X_CONTENT_TYPE_OPTIONS,
            http::HeaderValue::from_static("nosniff"),
        );
        headers.insert(http::header::CACHE_CONTROL, DOWNLOAD_CACHE_CONTROL);
        Ok(response)
    }
}

#[builder_impl]
impl Download {
    // **< Download BUILDER >***********************************************************************

    /// Establece el nombre con el que el navegador muestra o guarda el archivo, normalmente el
    /// original ([`UploadedFile::original_name()`](crate::web::UploadedFile::original_name)). Una
    /// cadena vacía usa el nombre del archivo guardado.
    pub fn with_file_name(mut self, file_name: impl AsRef<str>) -> Self {
        self.file_name = file_name.as_ref().trim().to_owned();
        self
    }

    /// Establece si el archivo se descarga siempre (`Content-Disposition: attachment`), aunque sea
    /// de un tipo que el navegador podría mostrar.
    pub fn with_attachment(mut self, attachment: bool) -> Self {
        self.attachment = attachment;
        self
    }
}

// **< Utilidades privadas >************************************************************************

// Tipos que el navegador muestra sin ejecutar nada en el contexto del sitio. SVG queda fuera porque
// es un documento que admite *scripts*, igual que HTML o XML.
fn shows_inline(content_type: &str) -> bool {
    let essence = content_type
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    match essence.split_once('/') {
        Some(("image", subtype)) => subtype != "svg+xml",
        Some(("audio" | "video", _)) => true,
        _ => matches!(essence.as_str(), "application/pdf" | "text/plain"),
    }
}

// `filename` lleva una versión ASCII del nombre para clientes antiguos, y `filename*` (RFC 6266)
// el nombre completo en UTF-8, que es el que usan los navegadores actuales.
fn content_disposition(inline: bool, file_name: &str) -> String {
    let name = sanitize_file_name(file_name);
    let mut value = String::with_capacity(32 + name.len() * 4);
    value.push_str(if inline { "inline" } else { "attachment" });
    value.push_str("; filename=\"");
    for c in name.chars() {
        let safe = c.is_ascii_alphanumeric() || matches!(c, ' ' | '.' | '-' | '_' | '(' | ')');
        value.push(if safe { c } else { '_' });
    }
    value.push_str("\"; filename*=UTF-8''");
    push_percent_encoded(&mut value, &name);
    value
}

#[cfg(test)]
mod tests {
    use super::{content_disposition, shows_inline};

    #[test]
    fn only_passive_types_are_shown_inline() {
        assert!(shows_inline("image/png"));
        assert!(shows_inline("video/mp4"));
        assert!(shows_inline("application/pdf"));
        assert!(shows_inline("text/plain; charset=utf-8"));
        assert!(!shows_inline("image/svg+xml"));
        assert!(!shows_inline("text/html"));
        assert!(!shows_inline("application/xhtml+xml"));
        assert!(!shows_inline("application/octet-stream"));
    }

    #[test]
    fn content_disposition_has_ascii_and_utf8_names() {
        assert_eq!(
            content_disposition(false, "Año \"2026\".pdf"),
            "attachment; filename=\"A_o _2026_.pdf\"; filename*=UTF-8''A%C3%B1o%20%222026%22.pdf"
        );
        assert_eq!(
            content_disposition(true, "../photo.jpg"),
            "inline; filename=\"photo.jpg\"; filename*=UTF-8''photo.jpg"
        );
    }
}
