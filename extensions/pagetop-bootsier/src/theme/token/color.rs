use pagetop::prelude::*;

// **< Color >**************************************************************************************

/// Paleta de colores temáticos.
///
/// Equivalen a los nombres estándar definidos por Bootstrap (`primary`, `secondary`, `success`,
/// etc.). Este tipo enumerado sirve de referencia para componer las clases de color para el fondo
/// ([`Background`](crate::theme::class::Background)), bordes
/// ([`Border`](crate::theme::class::Border)) o texto ([`Text`](crate::theme::class::Text)).
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
    /// Devuelve el nombre del color Bootstrap (`"primary"`, `"danger"`, etc.).
    #[rustfmt::skip]
    #[inline]
    pub const fn as_str(self) -> &'static str {
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
}

// **< Opacity >************************************************************************************

/// Niveles de opacidad (`opacity-*`).
///
/// Se usa normalmente para graduar la transparencia del color de fondo `bg-opacity-*`
/// ([`Background`](crate::theme::class::Background)), de los bordes `border-opacity-*`
/// ([`Border`](crate::theme::class::Border)) o del texto `text-opacity-*`
/// ([`Text`](crate::theme::class::Text)).
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
    // Devuelve el sufijo para `*opacity-*`, o `None` si no define ninguna clase.
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

    // Añade la opacidad a la cadena de clases usando el prefijo dado (`bg`, `border`, `text`, o
    // vacío para `opacity-*`).
    #[inline]
    pub(crate) fn push_to(self, classes: &mut String, prefix: &str) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            if prefix.is_empty() {
                classes.push_str("opacity");
            } else {
                classes.push_str(prefix);
                classes.push_str("-opacity");
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase de opacidad `opacity-*`.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// assert_eq!(token::Opacity::Opaque.to_class(), "opacity-100");
    /// assert_eq!(token::Opacity::Half.to_class(), "opacity-50");
    /// assert_eq!(token::Opacity::Default.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class, "");
        class
    }
}

// **< ColorBg >************************************************************************************

/// Esquema de color para el fondo ([`Background`](crate::theme::class::Background)).
///
/// - `Body`, `BodySecondary` y `BodyTertiary` siguen el esquema del tema (claro/oscuro).
/// - `Theme(Color)` y `Subtle(Color)` usan la paleta de colores temáticos ([`Color`]).
/// - `Black`, `White`, `Transparent` son colores fijos independientes del tema.
/// - `Default` no genera ninguna clase.
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
    /// Genera la clase `bg-{color}` (p. ej., `bg-primary`).
    Theme(Color),
    /// Genera la clase `bg-{color}-subtle` (un tono suavizado del color).
    Subtle(Color),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
    /// Fondo transparente (`bg-transparent`).
    Transparent,
}

impl ColorBg {
    // Devuelve el sufijo de la clase `bg-*`, o `None` si no define ninguna clase.
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

    // Añade la clase de fondo `bg-*` a la cadena de clases.
    #[inline]
    pub(crate) fn push_to(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Theme(c) | Self::Subtle(c) => {
                    classes.push_str("bg-");
                    classes.push_str(c.as_str());
                }
                _ => classes.push_str("bg"),
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase `bg-*` correspondiente al fondo.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let body = token::ColorBg::Body.to_class();
    /// assert_eq!(body, "bg-body");
    ///
    /// let theme = token::ColorBg::Theme(token::Color::Primary).to_class();
    /// assert_eq!(theme, "bg-primary");
    ///
    /// let subtle = token::ColorBg::Subtle(token::Color::Warning).to_class();
    /// assert_eq!(subtle, "bg-warning-subtle");
    ///
    /// let transparent = token::ColorBg::Transparent.to_class();
    /// assert_eq!(transparent, "bg-transparent");
    ///
    /// let none = token::ColorBg::Default.to_class();
    /// assert_eq!(none, "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl From<Color> for ColorBg {
    /// Convierte un [`Color`] en [`ColorBg::Theme`].
    ///
    /// Permite pasar un [`Color`] directamente donde se espera un [`ColorBg`], sin necesidad de
    /// envolver el valor en [`ColorBg::Theme`]. Es el atajo habitual para los colores temáticos.
    ///
    /// Para los demás esquemas (`Body`, `Subtle`, `Black`, etc.) se sigue usando [`ColorBg`]
    /// directamente.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let bg: token::ColorBg = token::Color::Primary.into();
    /// assert_eq!(bg.to_class(), "bg-primary");
    /// ```
    fn from(color: Color) -> Self {
        Self::Theme(color)
    }
}

// **< ColorText >**********************************************************************************

/// Esquema de color para el texto ([`Text`](crate::theme::class::Text)).
///
/// - `Body`, `BodyEmphasis`, `BodySecondary` y `BodyTertiary` siguen el tema (claro/oscuro).
/// - `Theme(Color)` y `Emphasis(Color)` usan la paleta de colores temáticos ([`Color`]).
/// - `Black` y `White` son colores fijos independientes del tema.
/// - `Default` no genera ninguna clase.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ColorText {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Color predefinido del tema (`text-body`).
    Body,
    /// Color de mayor contraste según el tema (`text-body-emphasis`).
    BodyEmphasis,
    /// Color predefinido del tema (`text-body-secondary`).
    BodySecondary,
    /// Color predefinido del tema (`text-body-tertiary`).
    BodyTertiary,
    /// Genera la clase `text-{color}`.
    Theme(Color),
    /// Genera la clase `text-{color}-emphasis` (mayor contraste acorde al tema).
    Emphasis(Color),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
}

impl ColorText {
    // Devuelve el sufijo de la clase `text-*`, o `None` si no define ninguna clase.
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

    // Añade la clase de texto `text-*` a la cadena de clases.
    #[inline]
    pub(crate) fn push_to(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Theme(c) | Self::Emphasis(c) => {
                    classes.push_str("text-");
                    classes.push_str(c.as_str());
                }
                _ => classes.push_str("text"),
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase `text-*` correspondiente al color del texto.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let body = token::ColorText::Body.to_class();
    /// assert_eq!(body, "text-body");
    ///
    /// let theme = token::ColorText::Theme(token::Color::Primary).to_class();
    /// assert_eq!(theme, "text-primary");
    ///
    /// let emphasis = token::ColorText::Emphasis(token::Color::Danger).to_class();
    /// assert_eq!(emphasis, "text-danger-emphasis");
    ///
    /// let black = token::ColorText::Black.to_class();
    /// assert_eq!(black, "text-black");
    ///
    /// let none = token::ColorText::Default.to_class();
    /// assert_eq!(none, "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl From<Color> for ColorText {
    /// Convierte un [`Color`] en [`ColorText::Theme`].
    ///
    /// Permite pasar un [`Color`] directamente donde se espera un [`ColorText`], sin necesidad de
    /// envolver el valor en [`ColorText::Theme`]. Es el atajo habitual para los colores temáticos.
    ///
    /// Para los demás esquemas (`Body`, `Emphasis`, `Black`, etc.) se sigue usando [`ColorText`]
    /// directamente.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let text: token::ColorText = token::Color::Danger.into();
    /// assert_eq!(text.to_class(), "text-danger");
    /// ```
    fn from(color: Color) -> Self {
        Self::Theme(color)
    }
}
