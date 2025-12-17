use pagetop::prelude::*;

// **< Color >**************************************************************************************

/// Paleta de colores temáticos.
///
/// Equivalen a los nombres estándar definidos por Bootstrap (`primary`, `secondary`, `success`,
/// etc.). Este tipo enumerado sirve de base para componer las clases de color para fondo
/// ([`classes::Background`](crate::theme::classes::Background)), bordes
/// ([`classes::Border`](crate::theme::classes::Border)) y texto
/// ([`classes::Text`](crate::theme::classes::Text)).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl Color {
    /// Devuelve el nombre del color.
    #[rustfmt::skip]
    #[inline]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Primary   => "primary",
            Self::Secondary => "secondary",
            Self::Success   => "success",
            Self::Info      => "info",
            Self::Warning   => "warning",
            Self::Danger    => "danger",
            Self::Light     => "light",
            Self::Dark      => "dark",
        }
    }

    /* Añade el nombre del color a la cadena de clases (reservado).
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(self.as_str());
    } */

    /// Devuelve la clase correspondiente al color.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(Color::Primary.to_class(), "primary");
    /// assert_eq!(Color::Danger.to_class(), "danger");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        self.as_str().to_owned()
    }
}

// **< Opacity >************************************************************************************

/// Niveles de opacidad (`opacity-*`).
///
/// Se usa normalmente para graduar la transparencia del color de fondo `bg-opacity-*`
/// ([`classes::Background`](crate::theme::classes::Background)), de los bordes `border-opacity-*`
/// ([`classes::Border`](crate::theme::classes::Border)) o del texto `text-opacity-*`
/// ([`classes::Text`](crate::theme::classes::Text)).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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
    const OPACITY: &str = "opacity";
    const OPACITY_PREFIX: &str = "-opacity";

    /// Devuelve el sufijo para `*opacity-*`, o `None` si no define ninguna clase.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Default           => None,
            Self::Opaque            => Some("-100"),
            Self::SemiOpaque        => Some("-75"),
            Self::Half              => Some("-50"),
            Self::SemiTransparent   => Some("-25"),
            Self::AlmostTransparent => Some("-10"),
            Self::Transparent       => Some("-0"),
        }
    }

    /// Añade la opacidad a la cadena de clases usando el prefijo dado (`bg`, `border`, `text`, o
    /// vacío para `opacity-*`).
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String, prefix: &str) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            if prefix.is_empty() {
                classes.push_str(Self::OPACITY);
            } else {
                classes.push_str(prefix);
                classes.push_str(Self::OPACITY_PREFIX);
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase de opacidad con el prefijo dado (`bg`, `border`, `text`, o vacío para
    /// `opacity-*`).
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(Opacity::Opaque.class_with(""), "opacity-100");
    /// assert_eq!(Opacity::Half.class_with("bg"), "bg-opacity-50");
    /// assert_eq!(Opacity::SemiTransparent.class_with("text"), "text-opacity-25");
    /// assert_eq!(Opacity::Default.class_with("bg"), "");
    /// ```
    #[doc(hidden)]
    pub fn class_with(self, prefix: &str) -> String {
        if let Some(suffix) = self.suffix() {
            let base_len = if prefix.is_empty() {
                Self::OPACITY.len()
            } else {
                prefix.len() + Self::OPACITY_PREFIX.len()
            };
            let mut class = String::with_capacity(base_len + suffix.len());
            if prefix.is_empty() {
                class.push_str(Self::OPACITY);
            } else {
                class.push_str(prefix);
                class.push_str(Self::OPACITY_PREFIX);
            }
            class.push_str(suffix);
            return class;
        }
        String::new()
    }

    /// Devuelve la clase de opacidad `opacity-*`.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(Opacity::Opaque.to_class(), "opacity-100");
    /// assert_eq!(Opacity::Half.to_class(), "opacity-50");
    /// assert_eq!(Opacity::Default.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        self.class_with("")
    }
}

// **< ColorBg >************************************************************************************

/// Colores `bg-*` para el fondo.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl ColorBg {
    const BG: &str = "bg";
    const BG_PREFIX: &str = "bg-";

    /// Devuelve el sufijo de la clase `bg-*`, o `None` si no define ninguna clase.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Default       => None,
            Self::Body          => Some("-body"),
            Self::BodySecondary => Some("-body-secondary"),
            Self::BodyTertiary  => Some("-body-tertiary"),
            Self::Theme(_)      => Some(""),
            Self::Subtle(_)     => Some("-subtle"),
            Self::Black         => Some("-black"),
            Self::White         => Some("-white"),
            Self::Transparent   => Some("-transparent"),
        }
    }

    /// Añade la clase de fondo `bg-*` a la cadena de clases.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Theme(c) | Self::Subtle(c) => {
                    classes.push_str(Self::BG_PREFIX);
                    classes.push_str(c.as_str());
                }
                _ => classes.push_str(Self::BG),
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase `bg-*` correspondiente al fondo.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(ColorBg::Body.to_class(), "bg-body");
    /// assert_eq!(ColorBg::Theme(Color::Primary).to_class(), "bg-primary");
    /// assert_eq!(ColorBg::Subtle(Color::Warning).to_class(), "bg-warning-subtle");
    /// assert_eq!(ColorBg::Transparent.to_class(), "bg-transparent");
    /// assert_eq!(ColorBg::Default.to_class(), "");
    /// ```
    pub fn to_class(self) -> String {
        if let Some(suffix) = self.suffix() {
            let base_len = match self {
                Self::Theme(c) | Self::Subtle(c) => Self::BG_PREFIX.len() + c.as_str().len(),
                _ => Self::BG.len(),
            };
            let mut class = String::with_capacity(base_len + suffix.len());
            match self {
                Self::Theme(c) | Self::Subtle(c) => {
                    class.push_str(Self::BG_PREFIX);
                    class.push_str(c.as_str());
                }
                _ => class.push_str(Self::BG),
            }
            class.push_str(suffix);
            return class;
        }
        String::new()
    }
}

// **< ColorText >**********************************************************************************

/// Colores `text-*` para el texto.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl ColorText {
    const TEXT: &str = "text";
    const TEXT_PREFIX: &str = "text-";

    /// Devuelve el sufijo de la clase `text-*`, o `None` si no define ninguna clase.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Default       => None,
            Self::Body          => Some("-body"),
            Self::BodyEmphasis  => Some("-body-emphasis"),
            Self::BodySecondary => Some("-body-secondary"),
            Self::BodyTertiary  => Some("-body-tertiary"),
            Self::Theme(_)      => Some(""),
            Self::Emphasis(_)   => Some("-emphasis"),
            Self::Black         => Some("-black"),
            Self::White         => Some("-white"),
        }
    }

    /// Añade la clase de texto `text-*` a la cadena de clases.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Theme(c) | Self::Emphasis(c) => {
                    classes.push_str(Self::TEXT_PREFIX);
                    classes.push_str(c.as_str());
                }
                _ => classes.push_str(Self::TEXT),
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase `text-*` correspondiente al color del texto.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(ColorText::Body.to_class(), "text-body");
    /// assert_eq!(ColorText::Theme(Color::Primary).to_class(), "text-primary");
    /// assert_eq!(ColorText::Emphasis(Color::Danger).to_class(), "text-danger-emphasis");
    /// assert_eq!(ColorText::Black.to_class(), "text-black");
    /// assert_eq!(ColorText::Default.to_class(), "");
    /// ```
    pub fn to_class(self) -> String {
        if let Some(suffix) = self.suffix() {
            let base_len = match self {
                Self::Theme(c) | Self::Emphasis(c) => Self::TEXT_PREFIX.len() + c.as_str().len(),
                _ => Self::TEXT.len(),
            };
            let mut class = String::with_capacity(base_len + suffix.len());
            match self {
                Self::Theme(c) | Self::Emphasis(c) => {
                    class.push_str(Self::TEXT_PREFIX);
                    class.push_str(c.as_str());
                }
                _ => class.push_str(Self::TEXT),
            }
            class.push_str(suffix);
            return class;
        }
        String::new()
    }
}
