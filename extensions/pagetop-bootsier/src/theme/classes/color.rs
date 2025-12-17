use pagetop::prelude::*;

use crate::theme::aux::{ColorBg, ColorText, Opacity};

// **< Background >*********************************************************************************

/// Clases para establecer **color/opacidad del fondo**.
///
/// # Ejemplos
///
/// ```
/// # use pagetop_bootsier::prelude::*;
/// // Sin clases.
/// let s = classes::Background::new();
/// assert_eq!(s.to_class(), "");
///
/// // Sólo color de fondo.
/// let s = classes::Background::with(ColorBg::Theme(Color::Primary));
/// assert_eq!(s.to_class(), "bg-primary");
///
/// // Color más opacidad.
/// let s = classes::Background::with(ColorBg::BodySecondary).with_opacity(Opacity::Half);
/// assert_eq!(s.to_class(), "bg-body-secondary bg-opacity-50");
///
/// // Usando `From<ColorBg>`.
/// let s: classes::Background = ColorBg::Black.into();
/// assert_eq!(s.to_class(), "bg-black");
///
/// // Usando `From<(ColorBg, Opacity)>`.
/// let s: classes::Background = (ColorBg::White, Opacity::SemiTransparent).into();
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
    pub fn with(color: ColorBg) -> Self {
        Self::default().with_color(color)
    }

    // **< Background BUILDER >*********************************************************************

    /// Establece el color de fondo (`bg-*`).
    pub fn with_color(mut self, color: ColorBg) -> Self {
        self.color = color;
        self
    }

    /// Establece la opacidad del fondo (`bg-opacity-*`).
    pub fn with_opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = opacity;
        self
    }

    // **< Background HELPERS >*********************************************************************

    /// Añade las clases de fondo a la cadena de clases.
    ///
    /// Concatena, en este orden, color del fondo (`bg-*`) y opacidad (`bg-opacity-*`),
    /// omitiendo los fragmentos vacíos.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        self.color.push_class(classes);
        self.opacity.push_class(classes, "bg");
    }

    /// Devuelve las clases de fondo como cadena (`"bg-primary"`, `"bg-body-secondary bg-opacity-50"`, etc.).
    ///
    /// Si no se define ni color ni opacidad, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut classes = String::new();
        self.push_class(&mut classes);
        classes
    }
}

impl From<(ColorBg, Opacity)> for Background {
    /// Atajo para crear un [`classes::Background`](crate::theme::classes::Background) a partir del color de fondo y
    /// la opacidad.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// # use pagetop_bootsier::prelude::*;
    /// let s: classes::Background = (ColorBg::White, Opacity::SemiTransparent).into();
    /// assert_eq!(s.to_class(), "bg-white bg-opacity-25");
    /// ```
    fn from((color, opacity): (ColorBg, Opacity)) -> Self {
        Background::with(color).with_opacity(opacity)
    }
}

impl From<ColorBg> for Background {
    /// Atajo para crear un [`classes::Background`](crate::theme::classes::Background) a partir del color de fondo.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// # use pagetop_bootsier::prelude::*;
    /// let s: classes::Background = ColorBg::Black.into();
    /// assert_eq!(s.to_class(), "bg-black");
    /// ```
    fn from(color: ColorBg) -> Self {
        Background::with(color)
    }
}

// **< Text >***************************************************************************************

/// Clases para establecer **color/opacidad del texto**.
///
/// # Ejemplos
///
/// ```
/// # use pagetop_bootsier::prelude::*;
/// // Sin clases.
/// let s = classes::Text::new();
/// assert_eq!(s.to_class(), "");
///
/// // Sólo color del texto.
/// let s = classes::Text::with(ColorText::Theme(Color::Primary));
/// assert_eq!(s.to_class(), "text-primary");
///
/// // Color del texto y opacidad.
/// let s = classes::Text::new().with_color(ColorText::White).with_opacity(Opacity::SemiTransparent);
/// assert_eq!(s.to_class(), "text-white text-opacity-25");
///
/// // Usando `From<ColorText>`.
/// let s: classes::Text = ColorText::Black.into();
/// assert_eq!(s.to_class(), "text-black");
///
/// // Usando `From<(ColorText, Opacity)>`.
/// let s: classes::Text = (ColorText::Theme(Color::Danger), Opacity::Opaque).into();
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
    pub fn with(color: ColorText) -> Self {
        Self::default().with_color(color)
    }

    // **< Text BUILDER >***************************************************************************

    /// Establece el color del texto (`text-*`).
    pub fn with_color(mut self, color: ColorText) -> Self {
        self.color = color;
        self
    }

    /// Establece la opacidad del texto (`text-opacity-*`).
    pub fn with_opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = opacity;
        self
    }

    // **< Text HELPERS >***************************************************************************

    /// Añade las clases de texto a la cadena de clases.
    ///
    /// Concatena, en este orden, `text-*` y `text-opacity-*`, omitiendo los fragmentos vacíos.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        self.color.push_class(classes);
        self.opacity.push_class(classes, "text");
    }

    /// Devuelve las clases de texto como cadena (`"text-primary"`, `"text-white text-opacity-25"`,
    /// etc.).
    ///
    /// Si no se define ni color ni opacidad, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut classes = String::new();
        self.push_class(&mut classes);
        classes
    }
}

impl From<(ColorText, Opacity)> for Text {
    /// Atajo para crear un [`classes::Text`](crate::theme::classes::Text) a partir del color del
    /// texto y su opacidad.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// # use pagetop_bootsier::prelude::*;
    /// let s: classes::Text = (ColorText::Theme(Color::Danger), Opacity::Opaque).into();
    /// assert_eq!(s.to_class(), "text-danger text-opacity-100");
    /// ```
    fn from((color, opacity): (ColorText, Opacity)) -> Self {
        Text::with(color).with_opacity(opacity)
    }
}

impl From<ColorText> for Text {
    /// Atajo para crear un [`classes::Text`](crate::theme::classes::Text) a partir del color del
    /// texto.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// # use pagetop_bootsier::prelude::*;
    /// let s: classes::Text = ColorText::Black.into();
    /// assert_eq!(s.to_class(), "text-black");
    /// ```
    fn from(color: ColorText) -> Self {
        Text::with(color)
    }
}
