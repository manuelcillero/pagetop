use pagetop::prelude::*;

use std::fmt;

// **< Color >**************************************************************************************

/// Paleta de colores **temáticos**.
///
/// Equivalente a los nombres estándar de Bootstrap (`primary`, `secondary`, `success`, etc.). Sirve
/// como base para componer clases de fondo ([`BgColor`]), borde ([`BorderColor`]) y texto
/// ([`TextColor`]).
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

// **< BgColor >************************************************************************************

/// Colores de fondo (`bg-*`).
///
/// - `Default` no añade clase (devuelve `""` para facilitar la composición de clases).
/// - `Body*` usa fondos predefinidos del tema (`bg-body`, `bg-body-secondary`, `bg-body-tertiary`).
/// - `Theme(Color)` genera `bg-{color}` (p. ej., `bg-primary`).
/// - `Subtle(Color)` genera `bg-{color}-subtle` (tono suave).
/// - `Black` y `White` son colores explícitos.
/// - `Transparent` no aplica color de fondo (`bg-transparent`).
#[derive(AutoDefault)]
pub enum BgColor {
    #[default]
    Default,
    Body,
    BodySecondary,
    BodyTertiary,
    Theme(Color),
    Subtle(Color),
    Black,
    White,
    Transparent,
}

#[rustfmt::skip]
impl fmt::Display for BgColor {
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

// **< BorderColor >********************************************************************************

/// Colores (`border-*`) para los bordes ([`Border`](crate::theme::aux::Border)).
///
/// - `Default` no añade clase (devuelve `""` para facilitar la composición de clases).
/// - `Theme(Color)` genera `border-{color}`.
/// - `Subtle(Color)` genera `border-{color}-subtle` (versión suavizada).
/// - `Black` y `White` son colores explícitos.
#[derive(AutoDefault)]
pub enum BorderColor {
    #[default]
    Default,
    Theme(Color),
    Subtle(Color),
    Black,
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

// **< TextColor >**********************************************************************************

/// Colores de texto y fondos de texto (`text-*`).
///
/// - `Default` no añade clase (devuelve `""` para facilitar la composición de clases).
/// - `Body*` aplica colores predefinidos del tema (`text-body`, `text-body-emphasis`,
///   `text-body-secondary`, `text-body-tertiary`).
/// - `Theme(Color)` genera `text-{color}`.
/// - `Emphasis(Color)` genera `text-{color}-emphasis` (contraste mayor acorde al tema).
/// - `Background(Color)` genera `text-bg-{color}` (para color de fondo del texto).
/// - `Black` y `White` son colores explícitos.
#[derive(AutoDefault)]
pub enum TextColor {
    #[default]
    Default,
    Body,
    BodyEmphasis,
    BodySecondary,
    BodyTertiary,
    Theme(Color),
    Emphasis(Color),
    Background(Color),
    Black,
    White,
}

#[rustfmt::skip]
impl fmt::Display for TextColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default       => Ok(()),
            Self::Body          => f.write_str("text-body"),
            Self::BodyEmphasis  => f.write_str("text-body-emphasis"),
            Self::BodySecondary => f.write_str("text-body-secondary"),
            Self::BodyTertiary  => f.write_str("text-body-tertiary"),
            Self::Theme(c)      => write!(f, "text-{c}"),
            Self::Emphasis(c)   => write!(f, "text-{c}-emphasis"),
            Self::Background(c) => write!(f, "text-bg-{c}"),
            Self::Black         => f.write_str("text-black"),
            Self::White         => f.write_str("text-white"),
        }
    }
}
