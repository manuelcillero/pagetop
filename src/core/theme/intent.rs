use crate::AutoDefault;

/// Intención semántica de un componente visual.
///
/// Representa la intención que pretende comunicar un componente (énfasis principal, confirmación de
/// un evento exitoso, aviso, peligro, etc.). Cada tema decidirá cómo pintarlo.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
}

impl Intent {
    /// Devuelve el nombre de la intención (`"primary"`, `"danger"`, etc.).
    #[rustfmt::skip]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Primary   => "primary",
            Self::Secondary => "secondary",
            Self::Success   => "success",
            Self::Info      => "info",
            Self::Warning   => "warning",
            Self::Danger    => "danger",
        }
    }
}
