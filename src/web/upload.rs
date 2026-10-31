use crate::prelude::*;

use crate::global;
use crate::web::multipart::{Field, MultipartError};
use crate::web::{Multipart, http};

use serde::de::DeserializeOwned;
use tokio::io::{AsyncWriteExt, BufWriter};

use std::io::ErrorKind;
use std::path::{Component as PathComponent, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

// Longitud máxima, en caracteres, del nombre original que se conserva de un archivo subido.
const MAX_NAME_CHARS: usize = 200;

// Longitud máxima de la extensión que se conserva en el nombre del archivo guardado.
const MAX_EXTENSION_CHARS: usize = 16;

// Tamaño del búfer de escritura de cada archivo. El cuerpo llega en trozos pequeños (de unos pocos
// KiB), y cada escritura de `tokio::fs::File` pasa por un hilo del *pool* de bloqueo: sin búfer, un
// archivo de 8 MiB tardaba 17 veces más en escribirse.
const WRITE_BUFFER_SIZE: usize = 256 * 1024;

// Intentos para encontrar un nombre libre antes de desistir; con nombres aleatorios de 64 bits una
// colisión es ya improbable.
const NAME_ATTEMPTS: usize = 4;

// **< UploadError >********************************************************************************

/// Errores al recibir, guardar o servir un archivo subido con [`Upload`] o [`Download`].
///
/// [`message()`](Self::message) da un mensaje traducido para mostrarlo junto al formulario, y
/// [`is_client_error()`](Self::is_client_error) separa los fallos del envío de los del servidor.
///
/// [`Download`]: crate::web::Download
#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    /// La parte del formulario no es un archivo, sino un campo de texto.
    #[error("form field `{0}` is not a file")]
    NotAFile(String),
    /// El archivo llega en un campo que no está entre los admitidos con
    /// [`Upload::with_file_fields()`].
    #[error("unexpected file field `{0}`")]
    UnexpectedField(String),
    /// El archivo supera el tamaño máximo permitido.
    #[error("file `{file}` exceeds the maximum size of {limit} bytes")]
    TooLarge {
        /// Nombre original del archivo, saneado.
        file: String,
        /// Tamaño máximo en bytes.
        limit: u64,
    },
    /// La extensión del archivo no está entre las permitidas.
    #[error("file `{file}` has an extension that is not allowed")]
    ExtensionNotAllowed {
        /// Nombre original del archivo, saneado.
        file: String,
        /// Extensiones permitidas, en minúsculas y sin el punto.
        allowed: Vec<String>,
    },
    /// El envío trae más archivos de los permitidos.
    #[error("too many files, the maximum is {limit}")]
    TooManyFiles {
        /// Número máximo de archivos.
        limit: usize,
    },
    /// Los campos de texto del envío superan en conjunto el tamaño máximo permitido
    /// ([`max_body_size`](crate::global::ServerSettings::max_body_size)).
    #[error("form fields exceed the maximum size of {limit} bytes")]
    FieldsTooLarge {
        /// Tamaño máximo en bytes.
        limit: usize,
    },
    /// Los campos de texto del envío no se corresponden con los del formulario esperado (falta
    /// alguno obligatorio, un valor no tiene el tipo esperado...).
    #[error("invalid form data: {0}")]
    InvalidForm(String),
    /// El archivo pedido no existe.
    #[error("file `{0}` not found")]
    NotFound(String),
    /// La ruta indicada sale del directorio de archivos subidos o no es relativa.
    #[error("invalid upload path `{0}`")]
    InvalidPath(String),
    /// Error al leer el cuerpo de la petición, incluido superar el límite de
    /// [`max_request_size`](crate::global::UploadSettings::max_request_size).
    #[error(transparent)]
    Multipart(#[from] MultipartError),
    /// Error al leer o escribir el archivo en disco.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl UploadError {
    /// Devuelve `true` si el error se debe a la petición (archivo demasiado grande, extensión no
    /// permitida, envío mal formado, archivo que no existe...) y no a un fallo del servidor.
    ///
    /// Permite decidir si se informa al usuario para que corrija el envío o se responde con un
    /// error interno.
    pub fn is_client_error(&self) -> bool {
        match self {
            Self::NotAFile(_)
            | Self::UnexpectedField(_)
            | Self::TooLarge { .. }
            | Self::ExtensionNotAllowed { .. }
            | Self::TooManyFiles { .. }
            | Self::FieldsTooLarge { .. }
            | Self::InvalidForm(_)
            | Self::NotFound(_) => true,
            Self::Multipart(err) => err.status().is_client_error(),
            Self::InvalidPath(_) | Self::Io(_) => false,
        }
    }

    /// Devuelve un mensaje traducido para el usuario, con el nombre del archivo y el límite
    /// superado cuando procede.
    ///
    /// Los errores del servidor dan un mensaje genérico, sin detalles internos.
    pub fn message(&self) -> Lc {
        match self {
            Self::TooLarge { file, limit } => Lc::l("upload_too_large")
                .with_arg("file", file.clone())
                .with_arg("size", format_size(*limit)),
            Self::ExtensionNotAllowed { file, allowed } => Lc::l("upload_extension_not_allowed")
                .with_arg("file", file.clone())
                .with_arg("allowed", allowed.join(", ")),
            Self::TooManyFiles { limit } => Lc::l("upload_too_many_files")
                .with_number("limit", u32::try_from(*limit).unwrap_or(u32::MAX)),
            Self::FieldsTooLarge { limit } => {
                Lc::l("upload_fields_too_large").with_arg("size", format_size(*limit as u64))
            }
            Self::NotFound(_) => Lc::l("upload_not_found"),
            Self::Multipart(err)
                if err.status() == http::StatusCode::PAYLOAD_TOO_LARGE
                    && global::SETTINGS.upload.max_request_size > 0 =>
            {
                Lc::l("upload_body_too_large").with_arg(
                    "size",
                    format_size(global::SETTINGS.upload.max_request_size as u64),
                )
            }
            err if err.is_client_error() => Lc::l("upload_invalid"),
            _ => Lc::l("upload_failed"),
        }
    }
}

// **< UploadedFile >*******************************************************************************

/// Archivo subido y guardado en el servidor por [`Upload::save()`] o [`Upload::receive()`].
#[derive(Clone, Debug, Getters)]
pub struct UploadedFile {
    /// Devuelve el nombre del campo del formulario en el que llegó el archivo.
    field: String,
    /// Devuelve la ruta del archivo guardado, relativa al directorio de archivos subidos y con `/`
    /// como separador (p. ej. `"photos/3f9c2a7d1b4e8a60.jpg"`).
    ///
    /// Es el valor que conviene guardar en la base de datos; [`Upload::local_path()`] la convierte
    /// en la ruta del archivo en disco y [`Download`](crate::web::Download) lo sirve.
    path: String,
    /// Devuelve el nombre original del archivo, saneado: sin directorios ni caracteres de control.
    ///
    /// Lo envía el navegador, así que sólo sirve para mostrarlo; nunca se usa para guardarlo.
    original_name: String,
    /// Devuelve el tipo MIME declarado por el navegador, si lo envía.
    ///
    /// No se comprueba contra el contenido del archivo, así que no debe tomarse como garantía.
    content_type: Option<String>,
    /// Devuelve el tamaño del archivo en bytes.
    #[getters(copy)]
    size: u64,
}

// **< UploadedForm >*******************************************************************************

/// Envío `multipart/form-data` recibido con [`Upload::receive()`]: los campos de texto
/// deserializados en `T` y los archivos ya guardados.
///
/// **Los archivos se borran al descartar el `UploadedForm`** (al terminar el handler, al salir
/// antes con `?`...) salvo que se conserven con [`keep()`](Self::keep). Basta con llamar a `keep()`
/// cuando el envío está procesado por completo (validado y registrado en la base de datos) para que
/// uno que no llegue a buen término no deje archivos huérfanos en disco.
#[derive(Debug)]
pub struct UploadedForm<T> {
    form: T,
    files: SavedFiles,
}

impl<T> UploadedForm<T> {
    /// Devuelve los campos de texto del formulario.
    pub fn form(&self) -> &T {
        &self.form
    }

    /// Devuelve los archivos guardados, en el orden en que llegaron.
    ///
    /// Los de un campo con varios archivos ([`form::File::with_multiple()`]) se obtienen filtrando
    /// por [`UploadedFile::field()`]: `files().iter().filter(|f| f.field() == "attachments")`.
    pub fn files(&self) -> &[UploadedFile] {
        &self.files.0
    }

    /// Devuelve el primer archivo guardado del campo `field`, o `None` si no se eligió ninguno.
    pub fn file(&self, field: impl AsRef<str>) -> Option<&UploadedFile> {
        let field = field.as_ref();
        self.files.0.iter().find(|file| file.field() == field)
    }

    /// Conserva en disco los archivos guardados y devuelve los campos de texto y los archivos.
    pub fn keep(self) -> (T, Vec<UploadedFile>) {
        let UploadedForm { form, mut files } = self;
        (form, std::mem::take(&mut files.0))
    }
}

// Archivo que se está escribiendo, que se borra si se descarta antes de `finish()`: al devolver un
// error y también si se cancela el futuro de `save()` (p. ej. al descartarse el handler).
struct PartialFile {
    local: PathBuf,
    file: Option<BufWriter<tokio::fs::File>>,
}

impl PartialFile {
    async fn write_all(&mut self, chunk: &[u8]) -> std::io::Result<()> {
        match self.file.as_mut() {
            Some(file) => file.write_all(chunk).await,
            None => Ok(()),
        }
    }

    async fn finish(&mut self) -> std::io::Result<()> {
        if let Some(file) = self.file.as_mut() {
            file.flush().await?;
        }
        self.file = None;
        Ok(())
    }
}

impl Drop for PartialFile {
    fn drop(&mut self) {
        // Se cierra antes de borrarlo, porque Windows no borra un archivo abierto.
        if self.file.take().is_none() {
            return;
        }
        if let Err(err) = std::fs::remove_file(&self.local)
            && err.kind() != ErrorKind::NotFound
        {
            trace::warn!(
                "Cannot remove incomplete upload `{}`: {err}",
                self.local.display()
            );
        }
    }
}

// Archivos guardados de un envío, que se borran al descartarse salvo que antes se vacíe la lista.
#[derive(Debug, Default)]
struct SavedFiles(Vec<UploadedFile>);

impl Drop for SavedFiles {
    // Borrado síncrono: `Drop` no puede esperar a una tarea asíncrona, y son pocos archivos (como
    // mucho `max_files`) que el sistema sólo tiene que desenlazar.
    fn drop(&mut self) {
        for file in self.0.drain(..) {
            let Ok(local) = Upload::local_path(file.path()) else {
                continue;
            };
            if let Err(err) = std::fs::remove_file(&local)
                && err.kind() != ErrorKind::NotFound
            {
                trace::warn!(
                    "Cannot remove discarded upload `{}`: {err}",
                    local.display()
                );
            }
        }
    }
}

// **< Upload >*************************************************************************************

/// Recibe y guarda en el servidor los archivos de un formulario `multipart/form-data`.
///
/// Los archivos se guardan en el directorio [`dir`](crate::global::UploadSettings::dir) de la
/// sección `[upload]` de la configuración, o en un subdirectorio suyo indicado con
/// [`with_subdir()`](Self::with_subdir). Cada archivo recibe un **nombre aleatorio** que conserva
/// la extensión del original: el nombre que envía el navegador no se usa nunca en disco, así que no
/// puede sobrescribir otros archivos ni salir del directorio. El nombre original saneado queda en
/// [`UploadedFile::original_name()`].
///
/// El archivo se escribe a medida que llega, sin cargarlo entero en memoria, y se descarta si
/// supera el tamaño máximo ([`max_file_size`](crate::global::UploadSettings::max_file_size) por
/// defecto) o si no llega completo. La suma de todos los archivos y campos del envío la limita
/// además [`max_request_size`](crate::global::UploadSettings::max_request_size), que el servidor
/// aplica a toda petición `multipart/form-data`.
///
/// Lo habitual es recibir el envío completo con [`receive()`](Self::receive), que deserializa los
/// campos de texto, limita el número de archivos y borra los guardados si el envío no llega a buen
/// término. [`save()`](Self::save) guarda una sola parte, para recorrer el envío a mano.
///
/// Para servir después los archivos con control de acceso, ver [`Download`](crate::web::Download).
///
/// # Ejemplo
///
/// Conviene definir la subida una sola vez, para que el formulario anuncie las mismas extensiones
/// (`accept`) que admite el servidor. El formulario necesita además [`form::Enctype::Multipart`]:
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// fn photo_upload() -> web::Upload {
///     web::Upload::new()
///         .with_subdir("photos")
///         .with_extensions(["jpg", "jpeg", "png"])
///         .with_file_fields(["photo"])
/// }
///
/// fn photo_form() -> Form {
///     Form::new()
///         .with_action("/garden/photos")
///         .with_enctype(form::Enctype::Multipart)
///         .with_child(
///             form::input::Field::text()
///                 .with_name("title")
///                 .with_label(Lc::n("Title")),
///         )
///         .with_child(
///             form::File::new()
///                 .with_name("photo")
///                 .with_label(Lc::n("Photo"))
///                 .with_accept(photo_upload().accept())
///                 .with_required(true),
///         )
///         .with_child(Button::submit(Lc::n("Upload")))
/// }
/// ```
///
/// El handler recibe el cuerpo con el extractor [`web::Multipart`] (siempre el último parámetro).
/// Los campos de texto se deserializan en una estructura propia, como con [`web::Form`], y los
/// archivos se conservan sólo si se llega a [`keep()`](UploadedForm::keep):
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// # fn photo_upload() -> web::Upload { web::Upload::new() }
///
/// #[derive(serde::Deserialize)]
/// struct PhotoForm {
///     title: String,
/// }
///
/// async fn upload_photo(
///     request: HttpRequest,
///     multipart: web::Multipart,
/// ) -> Result<Response, ErrorPage> {
///     let sent = match photo_upload().receive::<PhotoForm>(multipart).await {
///         Ok(sent) => sent,
///         Err(err) if err.is_client_error() => {
///             // Volver a mostrar el formulario con `err.message()`...
///             # todo!()
///         }
///         Err(_) => return Err(ErrorPage::InternalError(Some(request))),
///     };
///     let Some(photo) = sent.file("photo") else {
///         // Campo obligatorio sin archivo: volver a mostrar el formulario...
///         # todo!()
///     };
///     // Guardar `sent.form().title`, `photo.path()` y `photo.original_name()` en la base de
///     // datos. Si falla y se sale con `?`, el archivo se borra.
///     sent.keep();
///     Ok(Redirect::see_other("/garden/photos"))
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Upload {
    /// Devuelve el subdirectorio donde se guardan los archivos, relativo al de archivos subidos.
    /// Vacío, se guardan en el directorio raíz.
    subdir: String,
    /// Devuelve el tamaño máximo en bytes de cada archivo, si sustituye al de la configuración.
    #[getters(copy)]
    max_size: Option<u64>,
    /// Devuelve el número máximo de archivos de un envío, si sustituye al de la configuración.
    #[getters(copy)]
    max_files: Option<usize>,
    /// Devuelve las extensiones permitidas, en minúsculas y sin el punto. Vacío, se admite
    /// cualquiera.
    extensions: Vec<String>,
    /// Devuelve los nombres de los campos en los que se admiten archivos. Vacío, se admiten en
    /// cualquiera.
    file_fields: Vec<String>,
}

impl Upload {
    /// Crea un `Upload` que guarda en el directorio de archivos subidos, con los límites de la
    /// configuración y sin restringir las extensiones ni los campos.
    pub fn new() -> Self {
        Self::default()
    }

    /// Convierte la ruta de un archivo guardado ([`UploadedFile::path()`]) en su ruta en disco.
    ///
    /// Sirve para leer, servir o borrar el archivo más adelante. Devuelve
    /// [`UploadError::InvalidPath`] si la ruta no es relativa o sale del directorio de archivos
    /// subidos (p. ej. con `..`), para que una ruta manipulada no dé acceso a otros archivos.
    pub fn local_path(path: impl AsRef<str>) -> Result<PathBuf, UploadError> {
        let path = path.as_ref();
        let relative = checked_relative(path)?;
        if relative.as_os_str().is_empty() {
            return Err(UploadError::InvalidPath(path.to_owned()));
        }
        Ok(upload_dir().join(relative))
    }

    /// Borra un archivo guardado, a partir de su ruta ([`UploadedFile::path()`]).
    ///
    /// No es un error que el archivo ya no exista, para poder usarlo sin más comprobaciones al
    /// sustituir un archivo por otro o al borrar el registro que lo referencia. Igual que
    /// [`local_path()`](Self::local_path), rechaza las rutas que salen del directorio de archivos
    /// subidos.
    pub async fn remove(path: impl AsRef<str>) -> Result<(), UploadError> {
        match tokio::fs::remove_file(Self::local_path(path)?).await {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    /// Devuelve las extensiones permitidas con el formato del atributo `accept` de un campo
    /// [`form::File`] (p. ej. `".jpg,.png"`), o una cadena vacía si se admite cualquiera.
    ///
    /// Con `form::File::new().with_accept(upload.accept())` el formulario ofrece al usuario los
    /// mismos tipos de archivo que admite el servidor.
    pub fn accept(&self) -> String {
        let mut accept = String::new();
        for ext in self.extensions() {
            if !accept.is_empty() {
                accept.push(',');
            }
            accept.push('.');
            accept.push_str(ext);
        }
        accept
    }

    /// Recibe un envío `multipart/form-data` completo: guarda sus archivos y deserializa sus campos
    /// de texto en `T`, como hace [`web::Form`] en un formulario sin archivos.
    ///
    /// Los campos de texto se interpretan con `serde_qs`, igual que el cuerpo de un formulario
    /// normal leído con [`web::RawForm`] y `serde_qs::from_bytes()`. A diferencia de
    /// [`web::Form`], los repetidos (las casillas de una misma lista, p. ej.) se recogen en un
    /// `Vec`, marcado con `#[serde(default)]` para admitir que no llegue ninguno; en un campo
    /// simple repetido gana el último valor. Los nombres con corchetes (`tags[]`) se toman tal
    /// cual, sin anidar. En conjunto, los campos de texto no pueden superar
    /// [`max_body_size`](crate::global::ServerSettings::max_body_size), el mismo límite de un
    /// formulario sin archivos ([`UploadError::FieldsTooLarge`]).
    ///
    /// Cada parte con archivo se guarda con [`save()`](Self::save), con sus mismas comprobaciones,
    /// salvo las vacías (campos en los que no se eligió archivo), que se ignoran. Además, rechaza
    /// el envío si trae más archivos de [`max_files`](crate::global::UploadSettings::max_files) o
    /// de los indicados con [`with_max_files()`](Self::with_max_files).
    ///
    /// Si algo falla, borra los archivos ya guardados antes de devolver el error. Si no, el
    /// [`UploadedForm`] devuelto los borra también al descartarse, salvo que se conserven con
    /// [`UploadedForm::keep()`].
    pub async fn receive<T: DeserializeOwned>(
        &self,
        mut multipart: Multipart,
    ) -> Result<UploadedForm<T>, UploadError> {
        let max_files = self
            .max_files()
            .unwrap_or(global::SETTINGS.upload.max_files);
        let max_text = global::SETTINGS.server.max_body_size;
        let mut files = SavedFiles::default();
        // Los campos de texto se codifican como el cuerpo de un formulario normal a medida que
        // llegan, sin guardar antes cada valor entero.
        let mut fields = String::new();
        let mut text_size: usize = 0;

        while let Some(mut field) = multipart.next_field().await? {
            match field.file_name().map(str::is_empty) {
                Some(no_file) => {
                    if !no_file && max_files > 0 && files.0.len() >= max_files {
                        return Err(UploadError::TooManyFiles { limit: max_files });
                    }
                    if let Some(file) = self.save(field).await? {
                        files.0.push(file);
                    }
                }
                None => {
                    let Some(name) = field.name() else {
                        continue;
                    };
                    text_size += name.len() + 1;
                    if max_text > 0 && text_size > max_text {
                        return Err(UploadError::FieldsTooLarge { limit: max_text });
                    }
                    if !fields.is_empty() {
                        fields.push('&');
                    }
                    push_percent_encoded(&mut fields, name);
                    fields.push('=');
                    while let Some(chunk) = field.chunk().await? {
                        text_size += chunk.len();
                        if max_text > 0 && text_size > max_text {
                            return Err(UploadError::FieldsTooLarge { limit: max_text });
                        }
                        push_percent_encoded(&mut fields, &chunk);
                    }
                }
            }
        }

        let form =
            serde_qs::from_str(&fields).map_err(|err| UploadError::InvalidForm(err.to_string()))?;
        Ok(UploadedForm { form, files })
    }

    /// Guarda en disco el archivo de una parte del formulario.
    ///
    /// Devuelve `Ok(None)` si el campo llega sin archivo, como hace el navegador cuando no se elige
    /// ninguno en un campo opcional. Si el archivo no se puede guardar por completo (supera el
    /// tamaño máximo, se interrumpe la petición...), se borra lo escrito antes de devolver el
    /// error. También si se cancela antes de terminar, como cuando se descarta el handler.
    ///
    /// Cada archivo se guarda en cuanto llega su parte, así que si al recorrer el envío a mano
    /// falla una parte posterior, los anteriores ya están en disco: la aplicación debe borrarlos
    /// con [`Upload::remove()`]. [`receive()`](Self::receive) ya lo hace.
    ///
    /// El directorio del archivo (el de [`dir`](crate::global::UploadSettings::dir) más el
    /// subdirectorio de [`with_subdir()`](Self::with_subdir)) se crea si no existe. Si no se puede
    /// crear o no se puede escribir en él, devuelve [`UploadError::Io`] y deja constancia en el
    /// registro de trazas con la ruta afectada.
    pub async fn save(&self, mut field: Field<'_>) -> Result<Option<UploadedFile>, UploadError> {
        let field_name = field.name().unwrap_or_default().to_owned();
        let Some(file_name) = field.file_name() else {
            return Err(UploadError::NotAFile(field_name));
        };
        if !self.file_fields.is_empty() && !self.file_fields.contains(&field_name) {
            return Err(UploadError::UnexpectedField(field_name));
        }
        if file_name.is_empty() {
            return Ok(None);
        }
        let original_name = sanitize_file_name(file_name);
        let extension = file_extension(&original_name);
        if !self.extensions.is_empty()
            && !extension
                .as_ref()
                .is_some_and(|ext| self.extensions.contains(ext))
        {
            return Err(UploadError::ExtensionNotAllowed {
                file: original_name,
                allowed: self.extensions.clone(),
            });
        }
        let content_type = field.content_type().map(str::to_owned);

        let subdir = checked_relative(self.subdir())?;
        let dir = upload_dir().join(&subdir);
        // `io::Error` no incluye la ruta, así que se registra aquí para que el operador sepa qué
        // directorio falla (permisos, disco lleno...).
        if let Err(err) = tokio::fs::create_dir_all(&dir).await {
            trace::error!("Cannot create upload directory `{}`: {err}", dir.display());
            return Err(err.into());
        }
        let limit = self
            .max_size()
            .unwrap_or(global::SETTINGS.upload.max_file_size);
        let buffer_size = match usize::try_from(limit) {
            Ok(limit) if limit > 0 => limit.min(WRITE_BUFFER_SIZE),
            _ => WRITE_BUFFER_SIZE,
        };
        let (stored_name, local, file) = create_unique(&dir, extension.as_deref())
            .await
            .inspect_err(|err| {
                trace::error!("Cannot create upload file in `{}`: {err}", dir.display());
            })?;
        let mut partial = PartialFile {
            local,
            file: Some(BufWriter::with_capacity(buffer_size, file)),
        };

        let mut size: u64 = 0;
        while let Some(chunk) = field.chunk().await? {
            size += chunk.len() as u64;
            if limit > 0 && size > limit {
                return Err(UploadError::TooLarge {
                    file: original_name,
                    limit,
                });
            }
            partial.write_all(&chunk).await?;
        }
        partial.finish().await?;

        let mut path = String::new();
        for part in subdir.iter() {
            path.push_str(&part.to_string_lossy());
            path.push('/');
        }
        path.push_str(&stored_name);
        Ok(Some(UploadedFile {
            field: field_name,
            path,
            original_name,
            content_type,
            size,
        }))
    }
}

#[builder_impl]
impl Upload {
    // **< Upload BUILDER >*************************************************************************

    /// Establece el subdirectorio, relativo al de archivos subidos, donde se guardan los archivos
    /// (p. ej. `"photos"` o `"docs/2026"`). Una cadena vacía los guarda en el directorio raíz.
    ///
    /// No puede salir del directorio de archivos subidos: una ruta absoluta o con `..` hace que
    /// [`save()`](Self::save) devuelva [`UploadError::InvalidPath`].
    pub fn with_subdir(mut self, subdir: impl AsRef<str>) -> Self {
        self.subdir = subdir.as_ref().trim().to_owned();
        self
    }

    /// Establece el tamaño máximo en bytes de cada archivo. Con *0* sólo limita
    /// [`max_body_size`](crate::global::ServerSettings::max_body_size); pasa `None` para usar el
    /// de la configuración ([`max_file_size`](crate::global::UploadSettings::max_file_size)).
    pub fn with_max_size(mut self, max_size: impl Into<Option<u64>>) -> Self {
        self.max_size = max_size.into();
        self
    }

    /// Establece el número máximo de archivos que admite [`receive()`](Self::receive) en un mismo
    /// envío. Con *0* no se limita; pasa `None` para usar el de la configuración
    /// ([`max_files`](crate::global::UploadSettings::max_files)).
    pub fn with_max_files(mut self, max_files: impl Into<Option<usize>>) -> Self {
        self.max_files = max_files.into();
        self
    }

    /// Establece las extensiones permitidas, sin el punto y en mayúsculas o minúsculas (p. ej.
    /// `["jpg", "png"]`). Una lista vacía admite cualquier extensión.
    ///
    /// Se compara sólo la última extensión del nombre: un archivo `copia.tar.gz` tiene la extensión
    /// `gz`, y `informe.pdf.exe`, `exe`. Un archivo sin extensión, o con una que no sea sólo de
    /// letras y dígitos ASCII, se rechaza siempre que la lista no esté vacía.
    ///
    /// Conviene restringirlas siempre: aunque [`Download`](crate::web::Download) descarga sin
    /// mostrarlos los tipos que el navegador podría ejecutar, como `.html` o `.svg`, otras formas
    /// de publicar los archivos no tienen esa protección.
    pub fn with_extensions(
        mut self,
        extensions: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.extensions = extensions
            .into_iter()
            .map(|ext| {
                ext.as_ref()
                    .trim()
                    .trim_start_matches('.')
                    .to_ascii_lowercase()
            })
            .filter(|ext| !ext.is_empty())
            .collect();
        self
    }

    /// Establece los nombres de los campos del formulario en los que se admiten archivos (p. ej.
    /// `["photo"]`). Una lista vacía los admite en cualquiera.
    ///
    /// Un archivo en otro campo se rechaza con [`UploadError::UnexpectedField`]: un formulario
    /// legítimo nunca lo envía, y así no se guardan archivos que la aplicación no va a usar.
    pub fn with_file_fields(mut self, fields: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.file_fields = fields
            .into_iter()
            .map(|field| field.as_ref().trim().to_owned())
            .filter(|field| !field.is_empty())
            .collect();
        self
    }
}

// **< Utilidades privadas >************************************************************************

fn upload_dir() -> PathBuf {
    PathBuf::from(&global::SETTINGS.upload.dir)
}

// Acepta sólo componentes normales: ni raíz, ni prefijo de unidad, ni `..`; `.` se ignora.
fn checked_relative(path: &str) -> Result<PathBuf, UploadError> {
    let mut relative = PathBuf::new();
    for component in Path::new(path).components() {
        match component {
            PathComponent::Normal(part) => relative.push(part),
            PathComponent::CurDir => {}
            _ => return Err(UploadError::InvalidPath(path.to_owned())),
        }
    }
    Ok(relative)
}

// Quita los directorios (con `/` o `\`, según el sistema del navegador), los caracteres de control,
// los de control bidireccional y los puntos y espacios de los extremos, que ocultan el archivo o
// Windows descarta. Los bidireccionales invierten el texto al mostrarlo: `factura\u{202E}fdp.exe`
// se vería como «facturaexe.pdf».
pub(super) fn sanitize_file_name(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or_default();
    let clean: String = base
        .chars()
        .filter(|c| !c.is_control() && !is_bidi_control(*c))
        .take(MAX_NAME_CHARS)
        .collect();
    let clean = clean.trim_matches(|c: char| c == '.' || c.is_whitespace());
    if clean.is_empty() {
        "file".to_owned()
    } else {
        clean.to_owned()
    }
}

fn is_bidi_control(c: char) -> bool {
    matches!(
        c,
        '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}'
    )
}

// Extensión en minúsculas, si es corta y sólo tiene letras y dígitos ASCII; si no, ninguna, para
// que el nombre guardado no lleve nada que el navegador o el sistema puedan interpretar.
fn file_extension(name: &str) -> Option<String> {
    let (stem, ext) = name.rsplit_once('.')?;
    let valid = !stem.is_empty()
        && !ext.is_empty()
        && ext.len() <= MAX_EXTENSION_CHARS
        && ext.chars().all(|c| c.is_ascii_alphanumeric());
    valid.then(|| ext.to_ascii_lowercase())
}

// Codifica como `%XX` todos los bytes salvo los no reservados de RFC 3986, igual que un navegador
// codifica los campos de un formulario; vale también para `filename*` de `Content-Disposition`.
pub(super) fn push_percent_encoded(out: &mut String, value: impl AsRef<[u8]>) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for &byte in value.as_ref() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => {
                out.push('%');
                out.push(HEX[usize::from(byte >> 4)] as char);
                out.push(HEX[usize::from(byte & 0x0f)] as char);
            }
        }
    }
}

// Tamaño legible para los mensajes: en MiB si es un múltiplo exacto y, si no, en KiB redondeados
// hacia abajo, para no anunciar nunca un límite mayor que el real.
fn format_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    if bytes >= MIB && bytes.is_multiple_of(MIB) {
        util::join!((bytes / MIB).to_string(), " MiB")
    } else if bytes >= KIB {
        util::join!((bytes / KIB).to_string(), " KiB")
    } else {
        util::join!(bytes.to_string(), " B")
    }
}

// Nombre aleatorio de 16 cifras hexadecimales. `RandomState` toma sus claves de la aleatoriedad del
// sistema, así que los nombres no se pueden deducir aunque se conozcan la hora y el contador.
fn random_name() -> String {
    use std::hash::{BuildHasher, Hasher, RandomState};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u128(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default(),
    );
    hasher.write_u64(COUNTER.fetch_add(1, Ordering::Relaxed));
    format!("{:016x}", hasher.finish())
}

// Crea el archivo con un nombre que no exista ya (`create_new`), sin sobrescribir nunca otro.
async fn create_unique(
    dir: &Path,
    extension: Option<&str>,
) -> Result<(String, PathBuf, tokio::fs::File), UploadError> {
    let mut last_err = None;
    for _ in 0..NAME_ATTEMPTS {
        let name = match extension {
            Some(ext) => util::join!(random_name(), ".", ext),
            None => random_name(),
        };
        let local = dir.join(&name);
        match tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&local)
            .await
        {
            Ok(file) => return Ok((name, local, file)),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => last_err = Some(err),
            Err(err) => return Err(err.into()),
        }
    }
    Err(last_err
        .unwrap_or_else(|| ErrorKind::AlreadyExists.into())
        .into())
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_NAME_CHARS, PartialFile, checked_relative, file_extension, format_size,
        push_percent_encoded, random_name, sanitize_file_name,
    };
    use std::path::Path;
    use tokio::io::BufWriter;

    async fn partial_file() -> PartialFile {
        let local = std::env::temp_dir().join(random_name());
        let file = tokio::fs::File::create(&local).await.unwrap();
        PartialFile {
            local,
            file: Some(BufWriter::new(file)),
        }
    }

    #[tokio::test]
    async fn partial_file_is_removed_unless_finished() {
        let mut discarded = partial_file().await;
        discarded.write_all(b"half").await.unwrap();
        let local = discarded.local.clone();
        drop(discarded);
        assert!(!local.exists());

        let mut finished = partial_file().await;
        finished.write_all(b"whole").await.unwrap();
        finished.finish().await.unwrap();
        let local = finished.local.clone();
        drop(finished);
        assert_eq!(std::fs::read(&local).unwrap(), b"whole");
        std::fs::remove_file(local).unwrap();
    }

    #[test]
    fn sanitize_removes_directories_and_control_chars() {
        assert_eq!(sanitize_file_name("../../etc/passwd"), "passwd");
        assert_eq!(sanitize_file_name(r"C:\Users\me\photo.JPG"), "photo.JPG");
        assert_eq!(sanitize_file_name("re\u{0}port\n.pdf"), "report.pdf");
        assert_eq!(sanitize_file_name(" .hidden. "), "hidden");
        assert_eq!(sanitize_file_name("../"), "file");
        assert_eq!(
            sanitize_file_name("invoice\u{202E}fdp.exe"),
            "invoicefdp.exe"
        );
    }

    #[test]
    fn sanitize_limits_the_name_length() {
        let long = "a".repeat(MAX_NAME_CHARS + 50);
        assert_eq!(sanitize_file_name(&long).chars().count(), MAX_NAME_CHARS);
    }

    #[test]
    fn extension_is_lowercase_and_alphanumeric() {
        assert_eq!(file_extension("photo.JPG").as_deref(), Some("jpg"));
        assert_eq!(file_extension("archive.tar.gz").as_deref(), Some("gz"));
        assert_eq!(file_extension("noext"), None);
        assert_eq!(file_extension("bad.ex-t"), None);
        assert_eq!(file_extension("file.verylongextensionname"), None);
    }

    #[test]
    fn relative_paths_cannot_escape_the_upload_dir() {
        assert_eq!(
            checked_relative("photos/2026").unwrap(),
            Path::new("photos/2026")
        );
        assert_eq!(checked_relative("./photos").unwrap(), Path::new("photos"));
        assert!(checked_relative("../secret").is_err());
        assert!(checked_relative("photos/../../secret").is_err());
        assert!(checked_relative("/etc").is_err());
    }

    #[test]
    fn random_names_differ() {
        assert_ne!(random_name(), random_name());
        assert_eq!(random_name().len(), 16);
    }

    #[test]
    fn percent_encoding_keeps_only_unreserved_bytes() {
        let mut out = String::new();
        push_percent_encoded(&mut out, "tags[]=a b&ñ~._-");
        assert_eq!(out, "tags%5B%5D%3Da%20b%26%C3%B1~._-");
    }

    #[test]
    fn sizes_never_overstate_the_limit() {
        assert_eq!(format_size(8 * 1024 * 1024), "8 MiB");
        assert_eq!(format_size(1_500_000), "1464 KiB");
        assert_eq!(format_size(2048), "2 KiB");
        assert_eq!(format_size(500), "500 B");
    }
}
