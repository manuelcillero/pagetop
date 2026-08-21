use crate::AutoDefault;

// **< ColorName >**********************************************************************************

/// Interfaz común para los colores de la paleta de un tema.
///
/// PageTop ofrece una implementación predeterminada en [`CoreColors`], aunque probablemente cada
/// tema proporcionará su propia lista de colores implementando este trait.
pub trait ColorName {
    /// Devuelve el nombre asociado al color (p. ej. `"primary"`, `"danger"`, etc.). Normalmente se
    /// usará para generar la clase CSS del componente.
    fn name(self) -> &'static str;
}

// **< IntoColor >**********************************************************************************

/// Convierte un color, o su ausencia, en el nombre ya resuelto.
///
/// Permite que un método como `with_color(color: impl IntoColor)` acepte indistintamente un color
/// directo (`CoreColors::Danger`) o uno opcional (`Some(CoreColors::Danger)`, o `None` para no
/// aplicar ninguno).
///
/// Evita la ambigüedad de `impl<T> From<T> for T` junto con `impl<T> From<T> for Option<T>` al
/// resolver un `C` genérico acotado por [`ColorName`]. Al ser [`IntoColor`] un trait propio,
/// ninguna implementación de [`ColorName`] cubre nunca `Option<C>` y la resolución no es ambigua.
pub trait IntoColor {
    /// Devuelve el nombre del color ya resuelto, o `None` si no se aplica ninguno.
    fn into_color(self) -> Option<&'static str>;
}

impl<C: ColorName> IntoColor for C {
    fn into_color(self) -> Option<&'static str> {
        Some(self.name())
    }
}

impl<C: ColorName> IntoColor for Option<C> {
    fn into_color(self) -> Option<&'static str> {
        self.map(ColorName::name)
    }
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
