use pagetop::prelude::*;

use crate::theme::aux::Color;

use std::fmt;

// **< BorderColor >********************************************************************************

/// Colores `border-*` para los bordes ([`classes::Border`](crate::theme::classes::Border)).
#[derive(AutoDefault)]
pub enum BorderColor {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Genera internamente clases `border-{color}`.
    Theme(Color),
    /// Genera internamente clases `border-{color}-subtle` (un tono suavizado del color).
    Subtle(Color),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
}

#[rustfmt::skip]
impl fmt::Display for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default   => Ok(()),
            Self::Theme(c)  => write!(f, "border-{c}"),
            Self::Subtle(c) => write!(f, "border-{c}-subtle"),
            Self::Black     => f.write_str("border-black"),
            Self::White     => f.write_str("border-white"),
        }
    }
}

// **< BorderSize >*********************************************************************************

/// Tamaño para el ancho de los bordes ([`classes::Border`](crate::theme::classes::Border)).
///
/// Mapea a `border`, `border-0` y `border-{1..5}`:
///
/// - `None` no añade ninguna clase.
/// - `Default` genera `border` (borde por defecto del tema).
/// - `Zero` genera `border-0` (sin borde).
/// - `Scale{1..5}` genera `border-{1..5}` (ancho creciente).
#[derive(AutoDefault)]
pub enum BorderSize {
    #[default]
    None,
    Default,
    Zero,
    Scale1,
    Scale2,
    Scale3,
    Scale4,
    Scale5,
}

impl BorderSize {
    #[rustfmt::skip]
    pub(crate) fn to_class(&self, prefix: impl AsRef<str>) -> String {
        match self {
            Self::None    => String::new(),
            Self::Default => String::from(prefix.as_ref()),
            Self::Zero    => join!(prefix, "-0"),
            Self::Scale1  => join!(prefix, "-1"),
            Self::Scale2  => join!(prefix, "-2"),
            Self::Scale3  => join!(prefix, "-3"),
            Self::Scale4  => join!(prefix, "-4"),
            Self::Scale5  => join!(prefix, "-5"),
        }
    }
}

impl fmt::Display for BorderSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class("border"))
    }
}
