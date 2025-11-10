use pagetop::prelude::*;

use std::fmt;

// **< Color >**************************************************************************************

/// Paleta de colores temáticos.
///
/// Equivalen a los nombres estándar definidos por Bootstrap (`primary`, `secondary`, `success`,
/// etc.). Este tipo enumerado sirve de base para componer las clases de color para fondo
/// ([`classes::Background`](crate::theme::classes::Background)), bordes
/// ([`classes::Border`](crate::theme::classes::Border)) y texto
/// ([`classes::Text`](crate::theme::classes::Text)).
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

// **< Opacity >************************************************************************************

/// Niveles de opacidad (`opacity-*`).
///
/// Se usa normalmente para graduar la transparencia del color de fondo `bg-opacity-*`
/// ([`classes::Background`](crate::theme::classes::Background)), de los bordes `border-opacity-*`
/// ([`classes::Border`](crate::theme::classes::Border)) o del texto `text-opacity-*`
/// ([`classes::Text`](crate::theme::classes::Text)).
#[derive(AutoDefault)]
pub enum Opacity {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Permite generar clases `*-opacity-100` (100% de opacidad).
    Opaque,
    /// Permite generar clases `*-opacity-75` (75%).
    SemiOpaque,
    /// Permite generar clases `*-opacity-50` (50%).
    Half,
    /// Permite generar clases `*-opacity-25` (25%).
    SemiTransparent,
    /// Permite generar clases `*-opacity-10` (10%).
    AlmostTransparent,
    /// Permite generar clases `*-opacity-0` (0%, totalmente transparente).
    Transparent,
}

impl Opacity {
    #[rustfmt::skip]
    #[inline]
    const fn suffix(&self) -> &'static str {
        match self {
            Self::Default           => "",
            Self::Opaque            => "opacity-100",
            Self::SemiOpaque        => "opacity-75",
            Self::Half              => "opacity-50",
            Self::SemiTransparent   => "opacity-25",
            Self::AlmostTransparent => "opacity-10",
            Self::Transparent       => "opacity-0",
        }
    }

    #[inline]
    pub(crate) fn to_class(&self, prefix: impl AsRef<str>) -> String {
        match self {
            Self::Default => String::new(),
            _ => join_pair!(prefix, "-", self.suffix()),
        }
    }
}

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.suffix())
    }
}

// **< ColorBg >************************************************************************************

/// Colores `bg-*` para el fondo.
#[derive(AutoDefault)]
pub enum ColorBg {
    /// No define ninguna clase.
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

// **< ColorText >**********************************************************************************

/// Colores `text-*` para el texto.
#[derive(AutoDefault)]
pub enum ColorText {
    /// No define ninguna clase.
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
