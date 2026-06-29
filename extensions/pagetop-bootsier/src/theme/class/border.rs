use pagetop::prelude::*;

use crate::theme::token::{ColorBorder, Opacity, ScaleSize, Side};

/// Clases para definir **bordes**.
///
/// Se puede:
///
/// - Iniciar un borde sin tamaño inicial (`Border::default()`).
/// - Crear un borde con tamaño por defecto (`Border::new()`).
/// - Ajustar el tamaño de cada **lado lógico** (`side`, respetando LTR/RTL).
/// - Asignar **un tamaño global** para todo el borde (`size`).
/// - Aplicar un **color** al borde (`ColorBorder`).
/// - Aplicar un nivel de **opacidad** (`Opacity`).
///
/// # Comportamiento aditivo / sustractivo
///
/// - **Aditivo**: basta con crear un borde sin tamaño con `class::Border::default()` para ir
///   añadiendo cada lado lógico con el tamaño deseado usando `token::ScaleSize::{One..Five}`.
///
/// - **Sustractivo**: se crea un borde con tamaño predefinido, por ejemplo usando
///   `class::Border::new()` o `class::Border::with(token::ScaleSize::Two)` y eliminar los lados
///   deseados con `token::ScaleSize::Zero`.
///
/// - **Anchos diferentes por lado**: usando `token::ScaleSize::{Zero..Five}` en cada lado deseado.
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// // Borde global.
/// let b = class::Border::with(token::ScaleSize::Two);
/// assert_eq!(b.to_class(), "border-2");
///
/// // Aditivo (sólo borde superior):
/// let b = class::Border::default().with_side(token::Side::Top, token::ScaleSize::One);
/// assert_eq!(b.to_class(), "border-top-1");
///
/// // Sustractivo (borde global menos el superior):
/// let b = class::Border::new().with_side(token::Side::Top, token::ScaleSize::Zero);
/// assert_eq!(b.to_class(), "border border-top-0");
///
/// // Ancho por lado (lado lógico inicial a 2 y final a 4):
/// let b = class::Border::default()
///     .with_side(token::Side::Start, token::ScaleSize::Two)
///     .with_side(token::Side::End, token::ScaleSize::Four);
/// assert_eq!(b.to_class(), "border-end-4 border-start-2");
///
/// // Combinado (ejemplo completo):
/// let b = class::Border::new()                               // Borde por defecto.
///     .with_side(token::Side::Top, token::ScaleSize::Zero)   // Quita borde superior.
///     .with_side(token::Side::End, token::ScaleSize::Three)  // Ancho 3 para lado lógico final.
///     .with_color(token::Color::Primary)
///     .with_opacity(token::Opacity::Half);
/// assert_eq!(b.to_class(), "border border-top-0 border-end-3 border-primary border-opacity-50");
/// ```
#[rustfmt::skip]
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Border {
    all    : ScaleSize,
    top    : ScaleSize,
    end    : ScaleSize,
    bottom : ScaleSize,
    start  : ScaleSize,
    color  : ColorBorder,
    opacity: Opacity,
}

impl Border {
    /// Prepara un borde del tamaño predefinido. Equivale a `border` (ancho por defecto del tema).
    pub fn new() -> Self {
        Self::with(ScaleSize::Auto)
    }

    /// Define un borde con un tamaño global (`size`) para todos los lados.
    pub fn with(size: ScaleSize) -> Self {
        Self::default().with_side(Side::All, size)
    }

    // **< Border BUILDER >*************************************************************************

    /// Ajusta el tamaño del borde en el lado indicado (ver [`Side`](crate::theme::token::Side)).
    pub fn with_side(mut self, side: Side, size: ScaleSize) -> Self {
        match side {
            Side::All => self.all = size,
            Side::Top => self.top = size,
            Side::Bottom => self.bottom = size,
            Side::Start => self.start = size,
            Side::End => self.end = size,
            Side::LeftAndRight => {
                self.start = size;
                self.end = size;
            }
            Side::TopAndBottom => {
                self.top = size;
                self.bottom = size;
            }
        }
        self
    }

    /// Establece el color del borde.
    ///
    /// Acepta un tipo convertible en [`ColorBorder`]. Un [`Color`](crate::theme::token::Color) se
    /// convierte automáticamente en [`ColorBorder::Theme`].
    pub fn with_color(mut self, color: impl Into<ColorBorder>) -> Self {
        self.color = color.into();
        self
    }

    /// Establece la opacidad del borde.
    pub fn with_opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = opacity;
        self
    }

    // **< Border HELPERS >*************************************************************************

    /// Concatena, en este orden, las clases para *global*, `top`, `end`, `bottom`, `start`, *color*
    /// y *opacidad*; respetando LTR/RTL y omitiendo las definiciones vacías.
    #[rustfmt::skip]
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        self.all    .push_to(classes, "border");
        self.top    .push_to(classes, "border-top");
        self.end    .push_to(classes, "border-end");
        self.bottom .push_to(classes, "border-bottom");
        self.start  .push_to(classes, "border-start");
        self.color  .push_to(classes);
        self.opacity.push_to(classes, "border");
    }

    /// Devuelve las clases de borde como cadena (una combinación sencilla como `"border-2"`, o una
    /// más compleja como `"border border-top-0 border-end-3 border-primary border-opacity-50"`).
    ///
    /// Si no se define ningún tamaño, color ni opacidad, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut classes = String::new();
        self.push_to(&mut classes);
        classes
    }
}

impl From<ScaleSize> for Border {
    /// Crea un [`Border`] con un tamaño global para todos los lados.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// // Convertir explícitamente con `From::from`:
    /// let b = class::Border::from(token::ScaleSize::Two);
    /// assert_eq!(b.to_class(), "border-2");
    ///
    /// // Convertir implícitamente con `into()`:
    /// let b: class::Border = token::ScaleSize::Auto.into();
    /// assert_eq!(b.to_class(), "border");
    /// ```
    fn from(size: ScaleSize) -> Self {
        Self::with(size)
    }
}

impl Into<CowStr> for Border {
    /// Permite pasar [`Border`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}
