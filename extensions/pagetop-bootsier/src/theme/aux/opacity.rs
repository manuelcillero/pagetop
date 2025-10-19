use pagetop::prelude::*;

use std::fmt;

// **< Opacity >************************************************************************************

/// Niveles de **opacidad** (`opacity-*`).
///
/// Se usa para modular la transparencia del color de fondo `bg-opacity-*` ([`BgOpacity`]), borde
/// `border-opacity-*` ([`BorderOpacity`]) o texto `text-opacity-*` ([`TextOpacity`]), según las
/// siguientes equivalencias:
///
/// - `Opaque` => `opacity-100` (100% de opacidad).
/// - `SemiOpaque` => `opacity-75` (75%).
/// - `Half` => `opacity-50` (50%).
/// - `SemiTransparent` => `opacity-25` (25%).
/// - `AlmostTransparent` => `opacity-10` (10%).
/// - `Transparent` => `opacity-0` (0%, totalmente transparente).
#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum Opacity {
    #[default]
    Opaque,            // 100%
    SemiOpaque,        // 75%
    Half,              // 50%
    SemiTransparent,   // 25%
    AlmostTransparent, // 10%
    Transparent,       // 0%
}

#[rustfmt::skip]
impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Opaque            => f.write_str("opacity-100"),
            Self::SemiOpaque        => f.write_str("opacity-75"),
            Self::Half              => f.write_str("opacity-50"),
            Self::SemiTransparent   => f.write_str("opacity-25"),
            Self::AlmostTransparent => f.write_str("opacity-10"),
            Self::Transparent       => f.write_str("opacity-0"),
        }
    }
}

// **< BgOpacity >**********************************************************************************

/// Opacidad para el fondo (`bg-opacity-*`).
///
/// - `Default` no añade clase (devuelve `""` para facilitar la composición de clases).
/// - `Theme(Opacity)` genera `bg-{opacity}` (p. ej., `bg-opacity-50`).
#[derive(AutoDefault)]
pub enum BgOpacity {
    #[default]
    Default,
    Theme(Opacity),
}

impl fmt::Display for BgOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Theme(o) => write!(f, "bg-{o}"),
        }
    }
}

// **< BorderOpacity >******************************************************************************

/// Opacidad (`border-opacity-*`) para los bordes ([`Border`](crate::theme::aux::Border)).
///
/// - `Default` no añade clase (devuelve `""` para facilitar la composición de clases).
/// - `Theme(Opacity)` genera `border-{opacity}` (p. ej., `border-opacity-25`).
#[derive(AutoDefault)]
pub enum BorderOpacity {
    #[default]
    Default,
    Theme(Opacity),
}

impl fmt::Display for BorderOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Theme(o) => write!(f, "border-{o}"),
        }
    }
}

// **< TextOpacity >********************************************************************************

/// Opacidad para el texto (`text-opacity-*`).
///
/// - `Default` no añade clase (devuelve `""` para facilitar la composición de clases).
/// - `Theme(Opacity)` genera `text-{opacity}` (p. ej., `text-opacity-100`).
#[derive(AutoDefault)]
pub enum TextOpacity {
    #[default]
    Default,
    Theme(Opacity),
}

impl fmt::Display for TextOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Theme(o) => write!(f, "text-{o}"),
        }
    }
}
