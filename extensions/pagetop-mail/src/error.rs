//! Errores de `pagetop-mail`.

use std::path::PathBuf;

/// Errores al configurar el envío de correo o al componer y enviar un correo.
#[derive(Debug, thiserror::Error)]
pub enum MailError {
    /// Una dirección de correo no es válida.
    #[error("invalid email address \"{address}\"")]
    InvalidAddress {
        /// Dirección tal como se indicó.
        address: String,
        /// Motivo del error.
        #[source]
        source: lettre::address::AddressError,
    },
    /// Un ajuste de la sección `[mail]` no es válido.
    #[error("invalid mail setting \"{key}\"")]
    InvalidSetting {
        /// Clave del ajuste, p. ej. `"mail.from"`.
        key: &'static str,
        /// Motivo del error.
        #[source]
        source: Box<MailError>,
    },
    /// Una URL no es absoluta (no empieza por `http://` ni `https://`).
    #[error("invalid URL \"{0}\": it must start with http:// or https://")]
    InvalidUrl(String),
    /// El correo no indica remitente y no hay uno por defecto (`mail.from`).
    #[error("email has no sender: set mail.from or use Email::with_from()")]
    MissingFrom,
    /// El correo no tiene ningún destinatario (`To`, `Cc` ni `Bcc`).
    #[error("email has no recipients")]
    NoRecipients,
    /// El tipo de contenido de un adjunto no es un tipo MIME válido.
    #[error("invalid content type \"{0}\" for attachment")]
    InvalidContentType(String),
    /// No se puede leer el archivo de un adjunto.
    #[error("cannot read attachment \"{path}\"")]
    ReadAttachment {
        /// Ruta del archivo.
        path: PathBuf,
        /// Error devuelto al leerlo.
        #[source]
        source: std::io::Error,
    },
    /// No se puede crear el directorio de los archivos de correo (`mail.file.dir`).
    #[error("cannot create mail directory \"{path}\"")]
    CreateDir {
        /// Ruta del directorio.
        path: PathBuf,
        /// Error devuelto al crearlo.
        #[source]
        source: std::io::Error,
    },
    /// No se puede componer el mensaje.
    #[error("cannot build email message")]
    Build(#[source] lettre::error::Error),
    /// Error de conexión, de cifrado o de entrega con el servidor SMTP.
    #[error("SMTP error")]
    Smtp(#[source] lettre::transport::smtp::Error),
    /// Error al entregar el correo al programa *sendmail*.
    #[error("sendmail error")]
    Sendmail(#[source] lettre::transport::sendmail::Error),
    /// Error al guardar el correo como archivo.
    #[error("cannot write email file")]
    File(#[source] lettre::transport::file::Error),
}
