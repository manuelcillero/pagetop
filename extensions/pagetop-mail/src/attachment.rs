//! Archivos adjuntos e imágenes incrustadas ([`Attachment`]).

use pagetop::prelude::*;

use crate::MailError;

use lettre::message::SinglePart;
use lettre::message::header::ContentType;

use std::fmt;
use std::path::Path;
use std::sync::Arc;

/// Archivo adjunto a un correo, o recurso incrustado en su HTML.
///
/// Un adjunto tiene un nombre de archivo, un contenido y un tipo MIME. Si no se indica el tipo, se
/// deduce de la extensión del nombre (`informe.pdf` -> `application/pdf`) y, si no se reconoce, se
/// usa `application/octet-stream`.
///
/// Con [`with_content_id()`](Self::with_content_id) el adjunto pasa a ser un recurso **incrustado**
/// (*inline*), típicamente una imagen que el HTML del correo muestra con `src="cid:..."`. Sólo se
/// incrustan si el correo tiene HTML; si no, se envían como adjuntos normales.
///
/// El contenido se comparte al clonar el adjunto, sin copiarlo.
///
/// # Ejemplos
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_mail::prelude::*;
/// # async fn example(pdf: Vec<u8>, png: Vec<u8>) -> Result<(), MailError> {
/// // Un PDF generado por la aplicación.
/// let report = Attachment::new("informe.pdf", pdf);
///
/// // Un archivo del disco, con el nombre del archivo.
/// let terms = Attachment::from_path("docs/condiciones.pdf").await?;
///
/// // Un logotipo que el HTML muestra con <img src="cid:logo">.
/// let logo = Attachment::new("logo.png", png).with_content_id("logo");
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Getters)]
pub struct Attachment {
    /// Nombre del archivo, tal como lo verá el destinatario.
    filename: String,
    /// Tipo MIME del contenido.
    content_type: String,
    /// Identificador del recurso incrustado, si lo es.
    content_id: Option<String>,
    #[getters(skip)]
    body: Arc<[u8]>,
}

impl Attachment {
    /// Crea un adjunto con el nombre de archivo y el contenido dados.
    pub fn new(filename: impl Into<String>, body: impl Into<Vec<u8>>) -> Self {
        let filename = filename.into();
        let content_type = mime_guess::from_path(&filename)
            .first_or_octet_stream()
            .essence_str()
            .to_owned();
        Attachment {
            filename,
            content_type,
            content_id: None,
            body: Arc::from(body.into()),
        }
    }

    /// Crea un adjunto leyendo un archivo del disco. El nombre del adjunto es el del archivo.
    ///
    /// Falla con [`MailError::ReadAttachment`] si no se puede leer el archivo.
    pub async fn from_path(path: impl AsRef<Path>) -> Result<Self, MailError> {
        let path = path.as_ref();
        let body = tokio::fs::read(path)
            .await
            .map_err(|source| MailError::ReadAttachment {
                path: path.to_path_buf(),
                source,
            })?;
        let filename = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        Ok(Attachment::new(filename, body))
    }

    /// Contenido del adjunto.
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Indica si es un recurso incrustado en el HTML del correo.
    pub fn is_inline(&self) -> bool {
        self.content_id.is_some()
    }

    // Convierte el adjunto en una parte del mensaje. Los recursos incrustados sólo se incrustan si
    // `inline` es `true` (el correo tiene HTML).
    pub(crate) fn to_part(&self, inline: bool) -> Result<SinglePart, MailError> {
        let content_type = ContentType::parse(&self.content_type)
            .map_err(|_| MailError::InvalidContentType(self.content_type.clone()))?;
        let attachment = match self.content_id() {
            Some(id) if inline => lettre::message::Attachment::new_inline(id.clone()),
            _ => lettre::message::Attachment::new(self.filename.clone()),
        };
        Ok(attachment.body(self.body.to_vec(), content_type))
    }
}

// Muestra el tamaño del contenido en lugar de sus bytes.
impl fmt::Debug for Attachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attachment")
            .field("filename", &self.filename)
            .field("content_type", &self.content_type)
            .field("content_id", &self.content_id)
            .field("body_len", &self.body.len())
            .finish()
    }
}

#[builder_impl]
impl Attachment {
    // **< Attachment BUILDER >*********************************************************************

    /// Cambia el tipo MIME deducido del nombre del archivo (p. ej. `"text/csv; charset=utf-8"`).
    /// Si no es válido, el envío falla con [`MailError::InvalidContentType`].
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = content_type.into();
        self
    }

    /// Incrusta el adjunto en el HTML del correo con el identificador dado, que el HTML usa como
    /// `cid:<identificador>` (p. ej. `img src="cid:logo"`).
    pub fn with_content_id(mut self, content_id: impl Into<String>) -> Self {
        self.content_id = Some(content_id.into());
        self
    }
}
