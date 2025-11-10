use pagetop::prelude::*;

use crate::theme::aux::Color;

use std::fmt;

// **< ButtonColor >********************************************************************************

/// Variantes de color `btn-*` para botones.
#[derive(AutoDefault)]
pub enum ButtonColor {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Genera internamente clases `btn-{color}` (botón relleno).
    Background(Color),
    /// Genera `btn-outline-{color}` (fondo transparente y contorno con borde).
    Outline(Color),
    /// Aplica estilo de los enlaces (`btn-link`), sin caja ni fondo, heredando el color de texto.
    Link,
}

#[rustfmt::skip]
impl fmt::Display for ButtonColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default       => Ok(()),
            Self::Background(c) => write!(f, "btn-{c}"),
            Self::Outline(c)    => write!(f, "btn-outline-{c}"),
            Self::Link          => f.write_str("btn-link"),
        }
    }
}

// **< ButtonSize >*********************************************************************************

/// Tamaño visual de un botón.
#[derive(AutoDefault)]
pub enum ButtonSize {
    /// Tamaño por defecto del tema (no añade clase).
    #[default]
    Default,
    /// Botón compacto.
    Small,
    /// Botón destacado/grande.
    Large,
}

#[rustfmt::skip]
impl fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Small   => f.write_str("btn-sm"),
            Self::Large   => f.write_str("btn-lg"),
        }
    }
}
