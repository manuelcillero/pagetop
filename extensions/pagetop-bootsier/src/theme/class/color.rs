use pagetop::prelude::*;

use crate::theme::{OpacityLevel, ThemeColor};

// **< BgColor >************************************************************************************

/// Esquema de color para el fondo ([`Bg`]).
///
/// - `Body`, `BodySecondary` y `BodyTertiary` siguen el esquema del tema (claro/oscuro).
/// - `Solid(ThemeColor)` y `Subtle(ThemeColor)` usan la paleta de colores temáticos
///   ([`ThemeColor`]).
/// - `Black`, `White`, `Transparent` son colores fijos independientes del tema.
/// - `Default` no genera ninguna clase.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum BgColor {
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
    Solid(ThemeColor),
    /// Genera la clase `bg-{color}-subtle` (un tono suavizado del color).
    Subtle(ThemeColor),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
    /// Fondo transparente (`bg-transparent`).
    Transparent,
}

impl BgColor {
    // Devuelve el sufijo de la clase `bg-*`, o `None` si no define ninguna clase.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Default       => None,
            Self::Body          => Some("-body"),
            Self::BodySecondary => Some("-body-secondary"),
            Self::BodyTertiary  => Some("-body-tertiary"),
            Self::Solid(_)      => Some(""),
            Self::Subtle(_)     => Some("-subtle"),
            Self::Black         => Some("-black"),
            Self::White         => Some("-white"),
            Self::Transparent   => Some("-transparent"),
        }
    }

    /// Añade la clase de fondo `bg-*` a la cadena de clases.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Solid(c) | Self::Subtle(c) => {
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
    /// let body = class::BgColor::Body.to_class();
    /// assert_eq!(body, "bg-body");
    ///
    /// let solid = class::BgColor::Solid(ThemeColor::Primary).to_class();
    /// assert_eq!(solid, "bg-primary");
    ///
    /// let subtle = class::BgColor::Subtle(ThemeColor::Warning).to_class();
    /// assert_eq!(subtle, "bg-warning-subtle");
    ///
    /// let transparent = class::BgColor::Transparent.to_class();
    /// assert_eq!(transparent, "bg-transparent");
    ///
    /// let none = class::BgColor::Default.to_class();
    /// assert_eq!(none, "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl From<ThemeColor> for BgColor {
    /// Convierte un [`ThemeColor`] en [`BgColor::Solid`].
    ///
    /// Es el atajo habitual para los colores temáticos. Para los demás esquemas (`Body`, `Subtle`,
    /// `Black`, etc.) sigue usando [`BgColor`].
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let bg: class::BgColor = ThemeColor::Primary.into();
    /// assert_eq!(bg.to_class(), "bg-primary");
    /// ```
    fn from(color: ThemeColor) -> Self {
        Self::Solid(color)
    }
}

impl Into<CowStr> for BgColor {
    /// Permite pasar [`BgColor`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}

// **< Bg >*****************************************************************************************

/// Clases para establecer **color/opacidad del fondo**.
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// // Sin clases.
/// let s = class::Bg::new();
/// assert_eq!(s.to_class(), "");
///
/// // Sólo color de fondo (forma corta con ThemeColor).
/// let s = class::Bg::with(ThemeColor::Primary);
/// assert_eq!(s.to_class(), "bg-primary");
///
/// // Color más opacidad.
/// let s = class::Bg::with(class::BgColor::BodySecondary).with_opacity(OpacityLevel::Half);
/// assert_eq!(s.to_class(), "bg-body-secondary bg-opacity-50");
///
/// // Usando `From<BgColor>`.
/// let s: class::Bg = class::BgColor::Black.into();
/// assert_eq!(s.to_class(), "bg-black");
///
/// // Usando `From<(BgColor, OpacityLevel)>`.
/// let s: class::Bg = (class::BgColor::White, OpacityLevel::SemiTransparent).into();
/// assert_eq!(s.to_class(), "bg-white bg-opacity-25");
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Bg {
    color: BgColor,
    opacity: OpacityLevel,
}

impl Bg {
    /// Prepara un nuevo estilo para aplicar al fondo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un estilo fijando el color de fondo (`bg-*`).
    ///
    /// Acepta cualquier tipo convertible en [`BgColor`]. Un [`ThemeColor`] se convierte
    /// automáticamente en [`BgColor::Solid`]:
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// // Forma corta con ThemeColor:
    /// let s = class::Bg::with(ThemeColor::Primary);
    /// assert_eq!(s.to_class(), "bg-primary");
    ///
    /// // Forma explícita para variantes no temáticas:
    /// let s = class::Bg::with(class::BgColor::Body);
    /// assert_eq!(s.to_class(), "bg-body");
    /// ```
    pub fn with(color: impl Into<BgColor>) -> Self {
        Self::default().with_color(color)
    }

    // **< Bg BUILDER >*****************************************************************************

    /// Establece el color de fondo (`bg-*`).
    ///
    /// Acepta cualquier tipo convertible en [`BgColor`]. Un [`ThemeColor`] se convierte
    /// automáticamente en [`BgColor::Solid`].
    pub fn with_color(mut self, color: impl Into<BgColor>) -> Self {
        self.color = color.into();
        self
    }

    /// Establece la opacidad del fondo (`bg-opacity-*`).
    pub fn with_opacity(mut self, opacity: OpacityLevel) -> Self {
        self.opacity = opacity;
        self
    }

    // **< Bg HELPERS >*****************************************************************************

    /// Concatena, en este orden, color del fondo (`bg-*`) y opacidad (`bg-opacity-*`), omitiendo
    /// los fragmentos vacíos.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        self.color.push_to(classes);
        self.opacity.push_to(classes, "bg");
    }

    /// Devuelve las clases de fondo como cadena (`"bg-primary"`,
    /// `"bg-body-secondary bg-opacity-50"`, etc.).
    ///
    /// Si no se define ni color ni opacidad, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut classes = String::new();
        self.push_to(&mut classes);
        classes
    }
}

impl From<(BgColor, OpacityLevel)> for Bg {
    /// Crea la clase para un [`Bg`](crate::theme::class::Bg) a partir del color de fondo y la
    /// opacidad.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let s: class::Bg = (class::BgColor::White, OpacityLevel::SemiTransparent).into();
    /// assert_eq!(s.to_class(), "bg-white bg-opacity-25");
    /// ```
    fn from((color, opacity): (BgColor, OpacityLevel)) -> Self {
        Bg::with(color).with_opacity(opacity)
    }
}

impl From<BgColor> for Bg {
    /// Crea la clase para un [`Bg`](crate::theme::class::Bg) a partir del color de fondo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let s: class::Bg = class::BgColor::Black.into();
    /// assert_eq!(s.to_class(), "bg-black");
    /// ```
    fn from(color: BgColor) -> Self {
        Bg::with(color)
    }
}

impl Into<CowStr> for Bg {
    /// Permite pasar [`Bg`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}

// **< TextColor >**********************************************************************************

/// Esquema de color para el texto ([`Text`]).
///
/// - `Body`, `BodyEmphasis`, `BodySecondary` y `BodyTertiary` siguen el tema (claro/oscuro).
/// - `Solid(ThemeColor)` y `Emphasis(ThemeColor)` usan la paleta de colores temáticos
///   ([`ThemeColor`]).
/// - `Bg(ThemeColor)` genera la utilidad combinada `text-bg-{color}` (fondo más un color de texto
///   de contraste garantizado; no es una utilidad puramente de texto).
/// - `Black` y `White` son colores fijos independientes del tema.
/// - `Default` no genera ninguna clase.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum TextColor {
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
    Solid(ThemeColor),
    /// Genera la clase `text-{color}-emphasis` (mayor contraste acorde al tema).
    Emphasis(ThemeColor),
    /// Genera la clase `text-bg-{color}` (fondo con color de texto de contraste garantizado).
    Bg(ThemeColor),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
}

impl TextColor {
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
            Self::Solid(_)      => Some(""),
            Self::Emphasis(_)   => Some("-emphasis"),
            Self::Bg(_) => Some(""),
            Self::Black         => Some("-black"),
            Self::White         => Some("-white"),
        }
    }

    /// Añade la clase de texto `text-*` a la cadena de clases.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Solid(c) | Self::Emphasis(c) => {
                    classes.push_str("text-");
                    classes.push_str(c.as_str());
                }
                Self::Bg(c) => {
                    classes.push_str("text-bg-");
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
    /// let body = class::TextColor::Body.to_class();
    /// assert_eq!(body, "text-body");
    ///
    /// let solid = class::TextColor::Solid(ThemeColor::Primary).to_class();
    /// assert_eq!(solid, "text-primary");
    ///
    /// let emphasis = class::TextColor::Emphasis(ThemeColor::Danger).to_class();
    /// assert_eq!(emphasis, "text-danger-emphasis");
    ///
    /// let bg = class::TextColor::Bg(ThemeColor::Secondary).to_class();
    /// assert_eq!(bg, "text-bg-secondary");
    ///
    /// let black = class::TextColor::Black.to_class();
    /// assert_eq!(black, "text-black");
    ///
    /// let none = class::TextColor::Default.to_class();
    /// assert_eq!(none, "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl From<ThemeColor> for TextColor {
    /// Convierte un [`ThemeColor`] en [`TextColor::Solid`].
    ///
    /// Es el atajo habitual para los colores temáticos. Para los demás esquemas (`Body`,
    /// `Emphasis`, `Black`, etc.) sigue usando [`TextColor`].
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let text: class::TextColor = ThemeColor::Danger.into();
    /// assert_eq!(text.to_class(), "text-danger");
    /// ```
    fn from(color: ThemeColor) -> Self {
        Self::Solid(color)
    }
}

impl Into<CowStr> for TextColor {
    /// Permite pasar [`TextColor`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}

// **< Text >***************************************************************************************

/// Clases para establecer **color/opacidad del texto**.
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// // Sin clases.
/// let s = class::Text::new();
/// assert_eq!(s.to_class(), "");
///
/// // Sólo color del texto (forma corta con ThemeColor).
/// let s = class::Text::with(ThemeColor::Primary);
/// assert_eq!(s.to_class(), "text-primary");
///
/// // Color del texto y opacidad.
/// let s = class::Text::new().with_color(class::TextColor::White)
///     .with_opacity(OpacityLevel::SemiTransparent);
/// assert_eq!(s.to_class(), "text-white text-opacity-25");
///
/// // Usando `From<TextColor>`.
/// let s: class::Text = class::TextColor::Black.into();
/// assert_eq!(s.to_class(), "text-black");
///
/// // Usando `From<(TextColor, OpacityLevel)>`.
/// let s: class::Text = (
///     class::TextColor::Solid(ThemeColor::Danger),
///     OpacityLevel::Opaque,
/// ).into();
/// assert_eq!(s.to_class(), "text-danger text-opacity-100");
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Text {
    color: TextColor,
    opacity: OpacityLevel,
}

impl Text {
    /// Prepara un nuevo estilo para aplicar al texto.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un estilo fijando el color del texto (`text-*`).
    ///
    /// Acepta cualquier tipo convertible en [`TextColor`]. Un [`ThemeColor`] se convierte
    /// automáticamente en [`TextColor::Solid`]:
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// // Forma corta con ThemeColor:
    /// let s = class::Text::with(ThemeColor::Danger);
    /// assert_eq!(s.to_class(), "text-danger");
    ///
    /// // Forma explícita para variantes no temáticas:
    /// let s = class::Text::with(class::TextColor::Body);
    /// assert_eq!(s.to_class(), "text-body");
    /// ```
    pub fn with(color: impl Into<TextColor>) -> Self {
        Self::default().with_color(color)
    }

    // **< Text BUILDER >***************************************************************************

    /// Establece el color del texto (`text-*`).
    ///
    /// Acepta cualquier tipo convertible en [`TextColor`]. Un [`ThemeColor`] se convierte
    /// automáticamente en [`TextColor::Solid`].
    pub fn with_color(mut self, color: impl Into<TextColor>) -> Self {
        self.color = color.into();
        self
    }

    /// Establece la opacidad del texto (`text-opacity-*`).
    pub fn with_opacity(mut self, opacity: OpacityLevel) -> Self {
        self.opacity = opacity;
        self
    }

    // **< Text HELPERS >***************************************************************************

    /// Concatena, en este orden, `text-*` y `text-opacity-*`, omitiendo los fragmentos vacíos.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        self.color.push_to(classes);
        self.opacity.push_to(classes, "text");
    }

    /// Devuelve las clases de texto como cadena (`"text-primary"`, `"text-white text-opacity-25"`,
    /// etc.).
    ///
    /// Si no se define ni color ni opacidad, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut classes = String::new();
        self.push_to(&mut classes);
        classes
    }
}

impl From<(TextColor, OpacityLevel)> for Text {
    /// Crea la clase para [`Text`](crate::theme::class::Text) a partir del color del texto y su
    /// opacidad.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let s: class::Text = (
    ///     class::TextColor::Solid(ThemeColor::Danger),
    ///     OpacityLevel::Opaque,
    /// ).into();
    /// assert_eq!(s.to_class(), "text-danger text-opacity-100");
    /// ```
    fn from((color, opacity): (TextColor, OpacityLevel)) -> Self {
        Text::with(color).with_opacity(opacity)
    }
}

impl From<TextColor> for Text {
    /// Crea la clase para [`Text`](crate::theme::class::Text) a partir del color del texto.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let s: class::Text = class::TextColor::Black.into();
    /// assert_eq!(s.to_class(), "text-black");
    /// ```
    fn from(color: TextColor) -> Self {
        Text::with(color)
    }
}

impl Into<CowStr> for Text {
    /// Permite pasar [`Text`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}
