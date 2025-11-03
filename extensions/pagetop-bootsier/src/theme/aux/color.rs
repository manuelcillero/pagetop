use pagetop::prelude::*;

use std::fmt;

// **< Color >**************************************************************************************

/// Paleta de colores temáticos.
///
/// Equivalen a los nombres estándar definidos por Bootstrap (`primary`, `secondary`, `success`,
/// etc.). Este tipo enumerado sirve de base para componer clases de color para el fondo
/// ([`ColorBg`]), bordes ([`ColorBorder`]) y texto ([`ColorText`]).
#[derive(AutoDefault)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Light,
    Dark,
}

#[rustfmt::skip]
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primary   => f.write_str("primary"),
            Self::Secondary => f.write_str("secondary"),
            Self::Success   => f.write_str("success"),
            Self::Info      => f.write_str("info"),
            Self::Warning   => f.write_str("warning"),
            Self::Danger    => f.write_str("danger"),
            Self::Light     => f.write_str("light"),
            Self::Dark      => f.write_str("dark"),
        }
    }
}

// **< ColorBg >************************************************************************************

/// Colores `bg-*` para el fondo.
#[derive(AutoDefault)]
pub enum ColorBg {
    /// No define ninguna clase (devuelve `""` para facilitar la composición de clases).
    #[default]
    Default,
    /// Fondo predefinido del tema (`bg-body`).
    Body,
    /// Fondo predefinido del tema (`bg-body-secondary`).
    BodySecondary,
    /// Fondo predefinido del tema (`bg-body-tertiary`).
    BodyTertiary,
    /// Genera internamente clases `bg-{color}` (p. ej., `bg-primary`).
    Theme(Color),
    /// Genera internamente clases `bg-{color}-subtle` (un tono suavizado del color).
    Subtle(Color),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
    /// No aplica ningún color de fondo (`bg-transparent`).
    Transparent,
}

#[rustfmt::skip]
impl fmt::Display for ColorBg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default       => Ok(()),
            Self::Body          => f.write_str("bg-body"),
            Self::BodySecondary => f.write_str("bg-body-secondary"),
            Self::BodyTertiary  => f.write_str("bg-body-tertiary"),
            Self::Theme(c)      => write!(f, "bg-{c}"),
            Self::Subtle(c)     => write!(f, "bg-{c}-subtle"),
            Self::Black         => f.write_str("bg-black"),
            Self::White         => f.write_str("bg-white"),
            Self::Transparent   => f.write_str("bg-transparent"),
        }
    }
}

// **< ColorBorder >********************************************************************************

/// Colores `border-*` para los bordes ([`Border`](crate::theme::aux::Border)).
#[derive(AutoDefault)]
pub enum ColorBorder {
    /// No define ninguna clase (devuelve `""` para facilitar la composición de clases).
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
impl fmt::Display for ColorBorder {
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

// **< ColorButton >********************************************************************************

/// Variantes de color `btn-*` para botones.
#[derive(AutoDefault)]
pub enum ColorButton {
    /// No define ninguna clase (devuelve `""` para facilitar la composición de clases).
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
impl fmt::Display for ColorButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default       => Ok(()),
            Self::Background(c) => write!(f, "btn-{c}"),
            Self::Outline(c)    => write!(f, "btn-outline-{c}"),
            Self::Link          => f.write_str("btn-link"),
        }
    }
}

// **< ColorText >**********************************************************************************

/// Colores `text-*` para el texto.
#[derive(AutoDefault)]
pub enum ColorText {
    /// No define ninguna clase (devuelve `""` para facilitar la composición de clases).
    #[default]
    Default,
    /// Color predefinido del tema (`text-body`).
    Body,
    /// Color predefinido del tema (`text-body-emphasis`).
    BodyEmphasis,
    /// Color predefinido del tema (`text-body-secondary`).
    BodySecondary,
    /// Color predefinido del tema (`text-body-tertiary`).
    BodyTertiary,
    /// Genera internamente clases `text-{color}`.
    Theme(Color),
    /// Genera internamente clases `text-{color}-emphasis` (mayor contraste acorde al tema).
    Emphasis(Color),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
}

#[rustfmt::skip]
impl fmt::Display for ColorText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default       => Ok(()),
            Self::Body          => f.write_str("text-body"),
            Self::BodyEmphasis  => f.write_str("text-body-emphasis"),
            Self::BodySecondary => f.write_str("text-body-secondary"),
            Self::BodyTertiary  => f.write_str("text-body-tertiary"),
            Self::Theme(c)      => write!(f, "text-{c}"),
            Self::Emphasis(c)   => write!(f, "text-{c}-emphasis"),
            Self::Black         => f.write_str("text-black"),
            Self::White         => f.write_str("text-white"),
        }
    }
}

// **< Opacity >************************************************************************************

/// Niveles de opacidad (`opacity-*`).
///
/// Se usa normalmente para graduar la transparencia del color de fondo `bg-opacity-*`
/// ([`StyleBg`]), de los bordes `border-opacity-*` ([`StyleBorder`]) o del texto `text-opacity-*`
/// ([`StyleText`]).
#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum Opacity {
    /// Genera internamente clases `opacity-100` (100% de opacidad).
    #[default]
    Opaque,
    /// Genera internamente clases `opacity-75` (75%).
    SemiOpaque,
    /// Genera internamente clases `opacity-50` (50%).
    Half,
    /// Genera internamente clases `opacity-25` (25%).
    SemiTransparent,
    /// Genera internamente clases `opacity-10` (10%).
    AlmostTransparent,
    /// Genera internamente clases `opacity-0` (0%, totalmente transparente).
    Transparent,
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

// **< StyleBg >***********************************************************************************

/// Estilos de color/opacidad para el fondo.
#[derive(AutoDefault)]
pub enum StyleBg {
    /// No define ninguna clase (devuelve `""` para facilitar la composición de clases).
    #[default]
    Default,
    /// Genera internamente clases `bg-*`.
    Color(ColorBg),
    /// Genera internamente clases `bg-opacity-*`.
    Opacity(Opacity),
    /// Genera internamente clases `bg-* bg-opacity-*`.
    Both(ColorBg, Opacity),
}

#[rustfmt::skip]
impl fmt::Display for StyleBg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default    => Ok(()),
            Self::Color(c)   => write!(f, "{c}"),
            Self::Opacity(o) => write!(f, "bg-{o}"),
            Self::Both(c, o) => write!(f, "{c} bg-{o}"),
        }
    }
}

// **< StyleBorder >*******************************************************************************

/// Estilos de color/opacidad para los bordes ([`Border`](crate::theme::aux::Border)).
#[derive(AutoDefault)]
pub enum StyleBorder {
    /// No define ninguna clase (devuelve `""` para facilitar la composición de clases).
    #[default]
    Default,
    /// Genera internamente clases `border-*`.
    Color(ColorBorder),
    /// Genera internamente clases `border-opacity-*`.
    Opacity(Opacity),
    /// Genera internamente clases `border-* border-opacity-*`.
    Both(ColorBorder, Opacity),
}

#[rustfmt::skip]
impl fmt::Display for StyleBorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default    => Ok(()),
            Self::Color(c)   => write!(f, "{c}"),
            Self::Opacity(o) => write!(f, "border-{o}"),
            Self::Both(c, o) => write!(f, "{c} border-{o}"),
        }
    }
}

// **< StyleText >*********************************************************************************

/// Estilos de color/opacidad para texto y fondo del texto.
#[derive(AutoDefault)]
pub enum StyleText {
    /// No define ninguna clase (devuelve `""` para facilitar la composición de clases).
    #[default]
    Default,
    /// Genera internamente clases `text-*`.
    Color(ColorText),
    /// Genera internamente clases `text-opacity-*`.
    Opacity(Opacity),
    /// Genera internamente clases `text-* text-opacity-*`.
    Both(ColorText, Opacity),
    /// Genera internamente clases `text-bg-*` (para el color de fondo del texto).
    Bg(Color),
    /// Genera internamente clases `text-bg-* text-*`.
    BgAndColor(Color, ColorText),
    /// Genera internamente clases `text-bg-* text-* text-opacity-*`.
    All(Color, ColorText, Opacity),
}

#[rustfmt::skip]
impl fmt::Display for StyleText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default           => Ok(()),
            Self::Color(c)          => write!(f, "{c}"),
            Self::Opacity(o)        => write!(f, "text-{o}"),
            Self::Both(c, o)        => write!(f, "{c} text-{o}"),
            Self::Bg(bg)            => write!(f, "text-bg-{bg}"),
            Self::BgAndColor(bg, c) => write!(f, "text-bg-{bg} {c}"),
            Self::All(bg, c, o)     => write!(f, "text-bg-{bg} {c} text-{o}"),
        }
    }
}
