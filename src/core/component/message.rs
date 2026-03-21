use crate::locale::L10n;
use crate::{AutoDefault, Getters};

/// Nivel de severidad de un [`StatusMessage`].
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum MessageLevel {
    /// Mensaje informativo para el usuario.
    #[default]
    Info,
    /// Aviso o advertencia para el usuario.
    Warning,
    /// Error comunicado al usuario.
    Error,
}

/// Notificación amigable para el usuario generada al procesar una petición web.
///
/// Representa un mensaje con carácter informativo, una advertencia o un error. A diferencia de
/// [`ComponentError`](super::ComponentError), no está ligado a un fallo interno de renderizado,
/// puede generarse en cualquier punto del procesamiento de una petición web (manejadores,
/// renderizado, lógica de negocio, etc.).
///
/// El texto se almacena como [`L10n`] para resolverse con el idioma del contexto en el momento de
/// la visualización.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Mensaje informativo con clave traducible.
/// let info = StatusMessage::new(MessageLevel::Info, L10n::l("saved-successfully"));
///
/// // Aviso con texto literal sin traducción.
/// let warn = StatusMessage::new(MessageLevel::Warning, L10n::n("Formulario incompleto."));
/// ```
#[derive(Debug, Getters)]
pub struct StatusMessage {
    /// Nivel de severidad del mensaje.
    level: MessageLevel,
    /// Texto del mensaje.
    text: L10n,
}

impl StatusMessage {
    /// Crea un nuevo mensaje de usuario con el nivel y texto indicados.
    pub fn new(level: MessageLevel, text: L10n) -> Self {
        StatusMessage { level, text }
    }
}
