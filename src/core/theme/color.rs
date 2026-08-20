use crate::AutoDefault;

// **< ColorName >**********************************************************************************

/// Interfaz común para los colores de la paleta de un tema.
///
/// PageTop ofrece una implementación predeterminada en [`CoreColors`], aunque probablemente cada
/// tema proporcionará su propia lista de colores implementando este *trait*.
pub trait ColorName {
    /// Devuelve el nombre asociado al color (p. ej. `"primary"`, `"danger"`, etc.). Normalmente se
    /// usará para generar la clase CSS del componente.
    fn name(self) -> &'static str;
}

// **< CoreColors >*********************************************************************************

/// Paleta de colores predeterminada de PageTop.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum CoreColors {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
}

impl ColorName for CoreColors {
    fn name(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}
