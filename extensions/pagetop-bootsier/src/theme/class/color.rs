use pagetop::prelude::*;

use crate::theme::token::{ColorBg, ColorText, Opacity};

// **< Background >*********************************************************************************

/// Clases para establecer **color/opacidad del fondo**.
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// // Sin clases.
/// let s = class::Background::new();
/// assert_eq!(s.to_class(), "");
///
/// // Sólo color de fondo (forma corta con Color).
/// let s = class::Background::with(token::Color::Primary);
/// assert_eq!(s.to_class(), "bg-primary");
///
/// // Color más opacidad.
/// let s = class::Background::with(token::ColorBg::BodySecondary)
///     .with_opacity(token::Opacity::Half);
/// assert_eq!(s.to_class(), "bg-body-secondary bg-opacity-50");
///
/// // Usando `From<ColorBg>`.
/// let s: class::Background = token::ColorBg::Black.into();
/// assert_eq!(s.to_class(), "bg-black");
///
/// // Usando `From<(ColorBg, Opacity)>`.
/// let s: class::Background = (token::ColorBg::White, token::Opacity::SemiTransparent).into();
/// assert_eq!(s.to_class(), "bg-white bg-opacity-25");
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Background {
    color: ColorBg,
    opacity: Opacity,
}

impl Background {
    /// Prepara un nuevo estilo para aplicar al fondo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un estilo fijando el color de fondo (`bg-*`).
    ///
    /// Acepta cualquier tipo convertible en [`ColorBg`]. Un [`Color`](crate::theme::token::Color)
    /// se convierte automáticamente en [`ColorBg::Theme`]:
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// // Forma corta con Color:
    /// let s = class::Background::with(token::Color::Primary);
    /// assert_eq!(s.to_class(), "bg-primary");
    ///
    /// // Forma explícita para variantes no temáticas:
    /// let s = class::Background::with(token::ColorBg::Body);
    /// assert_eq!(s.to_class(), "bg-body");
    /// ```
    pub fn with(color: impl Into<ColorBg>) -> Self {
        Self::default().with_color(color)
    }

    // **< Background BUILDER >*********************************************************************

    /// Establece el color de fondo (`bg-*`).
    ///
    /// Acepta cualquier tipo convertible en [`ColorBg`]. Un [`Color`](crate::theme::token::Color)
    /// se convierte automáticamente en [`ColorBg::Theme`].
    pub fn with_color(mut self, color: impl Into<ColorBg>) -> Self {
        self.color = color.into();
        self
    }

    /// Establece la opacidad del fondo (`bg-opacity-*`).
    pub fn with_opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = opacity;
        self
    }

    // **< Background HELPERS >*********************************************************************

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

impl From<(ColorBg, Opacity)> for Background {
    /// Crea la clase para un [`Background`](crate::theme::class::Background) a partir del color de
    /// fondo y la opacidad.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let s: class::Background = (token::ColorBg::White, token::Opacity::SemiTransparent).into();
    /// assert_eq!(s.to_class(), "bg-white bg-opacity-25");
    /// ```
    fn from((color, opacity): (ColorBg, Opacity)) -> Self {
        Background::with(color).with_opacity(opacity)
    }
}

impl From<ColorBg> for Background {
    /// Crea la clase para un [`Background`](crate::theme::class::Background) a partir del color de
    /// fondo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let s: class::Background = token::ColorBg::Black.into();
    /// assert_eq!(s.to_class(), "bg-black");
    /// ```
    fn from(color: ColorBg) -> Self {
        Background::with(color)
    }
}

impl Into<CowStr> for Background {
    /// Permite pasar [`Background`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
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
/// // Sólo color del texto (forma corta con Color).
/// let s = class::Text::with(token::Color::Primary);
/// assert_eq!(s.to_class(), "text-primary");
///
/// // Color del texto y opacidad.
/// let s = class::Text::new().with_color(token::ColorText::White)
///     .with_opacity(token::Opacity::SemiTransparent);
/// assert_eq!(s.to_class(), "text-white text-opacity-25");
///
/// // Usando `From<ColorText>`.
/// let s: class::Text = token::ColorText::Black.into();
/// assert_eq!(s.to_class(), "text-black");
///
/// // Usando `From<(ColorText, Opacity)>`.
/// let s: class::Text = (
///     token::ColorText::Theme(token::Color::Danger),
///     token::Opacity::Opaque,
/// ).into();
/// assert_eq!(s.to_class(), "text-danger text-opacity-100");
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Text {
    color: ColorText,
    opacity: Opacity,
}

impl Text {
    /// Prepara un nuevo estilo para aplicar al texto.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un estilo fijando el color del texto (`text-*`).
    ///
    /// Acepta cualquier tipo convertible en [`ColorText`]. Un [`Color`](crate::theme::token::Color)
    /// se convierte automáticamente en [`ColorText::Theme`]:
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// // Forma corta con Color:
    /// let s = class::Text::with(token::Color::Danger);
    /// assert_eq!(s.to_class(), "text-danger");
    ///
    /// // Forma explícita para variantes no temáticas:
    /// let s = class::Text::with(token::ColorText::Body);
    /// assert_eq!(s.to_class(), "text-body");
    /// ```
    pub fn with(color: impl Into<ColorText>) -> Self {
        Self::default().with_color(color)
    }

    // **< Text BUILDER >***************************************************************************

    /// Establece el color del texto (`text-*`).
    ///
    /// Acepta cualquier tipo convertible en [`ColorText`]. Un [`Color`](crate::theme::token::Color)
    /// se convierte automáticamente en [`ColorText::Theme`].
    pub fn with_color(mut self, color: impl Into<ColorText>) -> Self {
        self.color = color.into();
        self
    }

    /// Establece la opacidad del texto (`text-opacity-*`).
    pub fn with_opacity(mut self, opacity: Opacity) -> Self {
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

impl From<(ColorText, Opacity)> for Text {
    /// Crea la clase para [`Text`](crate::theme::class::Text) a partir del color del texto y su
    /// opacidad.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let s: class::Text = (
    ///     token::ColorText::Theme(token::Color::Danger),
    ///     token::Opacity::Opaque,
    /// ).into();
    /// assert_eq!(s.to_class(), "text-danger text-opacity-100");
    /// ```
    fn from((color, opacity): (ColorText, Opacity)) -> Self {
        Text::with(color).with_opacity(opacity)
    }
}

impl From<ColorText> for Text {
    /// Crea la clase para [`Text`](crate::theme::class::Text) a partir del color del texto.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let s: class::Text = token::ColorText::Black.into();
    /// assert_eq!(s.to_class(), "text-black");
    /// ```
    fn from(color: ColorText) -> Self {
        Text::with(color)
    }
}

impl Into<CowStr> for Text {
    /// Permite pasar [`Text`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}
