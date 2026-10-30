//! Errores de `pagetop-mail`.

use std::path::PathBuf;

/// Errores al configurar el envío de correo o al componer y enviar un correo.
///
/// Los devuelven [`Email::send()`](crate::Email::send) y, con la *feature* `queue`,
/// `Email::queue()`. [`Email::send_later()`](crate::Email::send_later) no devuelve nada: registra
/// sus errores en las trazas. Un ajuste no válido ([`InvalidSetting`](Self::InvalidSetting))
/// detiene la aplicación al arrancar, así que sólo llega a un envío fuera de una aplicación (p. ej.
/// en pruebas que no la preparan).
///
/// El mensaje de cada error (`Display`) describe sólo el error principal; el motivo concreto, como
/// la respuesta del servidor SMTP, está en sus causas, que se recorren con
/// [`source()`](std::error::Error::source).
///
/// # Variantes futuras
///
/// El tipo está marcado con `#[non_exhaustive]`: nuevas versiones pueden añadir variantes sin que
/// sea un cambio incompatible, y algunas sólo existen con una *feature* (`Queue`, con `queue`). Por
/// eso, fuera de este *crate*, un `match` sobre `MailError` debe incluir siempre un brazo comodín
/// (`_`), aunque enumere todas las variantes que conoce. Para tratar sólo algunos casos también
/// sirven `if let` y `matches!`.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_mail::prelude::*;
///
/// async fn invite(address: String) -> Result<(), String> {
///     let result = Email::new()
///         .with_to(address.as_str())
///         .with_subject("Invitación")
///         .with_text("Te invitamos a unirte al proyecto.")
///         .send()
///         .await;
///     match result {
///         Ok(()) => Ok(()),
///         // Un error que puede corregir quien escribió la dirección.
///         Err(MailError::InvalidAddress { .. }) => Err(format!("\"{address}\" no es válida")),
///         // El resto, y cualquier variante futura, es un fallo interno.
///         Err(err) => {
///             trace::error!("Cannot send invitation: {err}");
///             Err("No se ha podido enviar la invitación".to_owned())
///         }
///     }
/// }
/// ```
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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
    /// Un valor numérico de la configuración está fuera del rango admitido (causa de
    /// [`InvalidSetting`](Self::InvalidSetting)).
    #[error("{value} is not between {min} and {max}")]
    OutOfRange {
        /// Valor indicado.
        value: u64,
        /// Valor mínimo admitido.
        min: u64,
        /// Valor máximo admitido.
        max: u64,
    },
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
    /// La entrega no ha terminado en el tiempo máximo, en segundos
    /// ([`MailSettings::delivery_timeout_secs`]): el servidor SMTP o el programa *sendmail* no
    /// responden, o el correo esperó su turno de envío todo ese tiempo (ver
    /// [`MailSettings::max_waiting`]). Es un fallo temporal; si el envío llegó a empezar, el correo
    /// puede haberse entregado igualmente.
    ///
    /// [`MailSettings::delivery_timeout_secs`]: crate::config::MailSettings::delivery_timeout_secs
    /// [`MailSettings::max_waiting`]: crate::config::MailSettings::max_waiting
    #[error("email delivery did not finish within {0} seconds")]
    Timeout(u64),
    /// Hay demasiados correos esperando su turno de envío y la entrega se rechaza enseguida, sin
    /// empezar, para no acumularlos en memoria: llegan más correos de los que el servidor despacha
    /// (ver [`MailSettings::max_waiting`](crate::config::MailSettings::max_waiting)). Es un fallo
    /// temporal.
    #[error("too many emails waiting to be delivered")]
    Overloaded,
    /// La entrega no ha llegado a empezar porque la aplicación se está deteniendo. Es un fallo
    /// temporal.
    #[error("email delivery interrupted: the application is shutting down")]
    Interrupted,
    /// Error de la base de datos al guardar el correo en la cola de envío (*feature* `queue`).
    #[cfg(feature = "queue")]
    #[error("cannot store email in the mail queue")]
    Queue(#[source] pagetop_seaorm::db::DbErr),
}

impl MailError {
    // Indica si el fallo es definitivo y no tiene sentido reintentar el envío: el servidor SMTP
    // rechaza el correo (5xx, p. ej. una dirección inexistente) o el sobre guardado en la cola no
    // es válido. Los fallos de conexión, de cifrado, los temporales (4xx) y los tiempos agotados se
    // pueden reintentar.
    #[cfg(feature = "queue")]
    pub(crate) fn is_permanent(&self) -> bool {
        match self {
            MailError::Smtp(err) => err.is_permanent(),
            MailError::InvalidAddress { .. } | MailError::Build(_) => true,
            _ => false,
        }
    }

    // Indica si el fallo no depende del correo sino del transporte, y afectaría igual a los
    // siguientes: no hay respuesta del servidor SMTP (conexión, cifrado, configuración, tiempo
    // agotado...), hay demasiados correos esperando turno, o fallan *sendmail* o los archivos. Las
    // respuestas del servidor (4xx y 5xx) dependen de cada correo, p. ej. de sus destinatarios.
    #[cfg(feature = "queue")]
    pub(crate) fn is_transport_failure(&self) -> bool {
        match self {
            MailError::Smtp(err) => !err.is_transient() && !err.is_permanent(),
            MailError::Sendmail(_)
            | MailError::File(_)
            | MailError::Timeout(_)
            | MailError::Overloaded
            | MailError::Interrupted => true,
            _ => false,
        }
    }

    // Describe el error con todas sus causas, para las trazas.
    pub(crate) fn describe(&self) -> String {
        let mut message = self.to_string();
        let mut source = std::error::Error::source(self);
        while let Some(cause) = source {
            message.push_str(": ");
            message.push_str(&cause.to_string());
            source = cause.source();
        }
        message
    }
}
