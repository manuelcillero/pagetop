use pagetop::prelude::*;

use crate::theme::aux::{BorderColor, Opacity, ScaleSize, Side};

/// Clases para crear **bordes**.
///
/// Permite:
///
/// - Iniciar un borde sin tamaño inicial (`Border::default()`).
/// - Crear un borde con tamaño por defecto (`Border::new()`).
/// - Ajustar el tamaño de cada **lado lógico** (`side`, respetando LTR/RTL).
/// - Definir un tamaño **global** para todo el borde (`size`).
/// - Aplicar un **color** al borde (`BorderColor`).
/// - Aplicar un nivel de **opacidad** (`Opacity`).
///
/// # Comportamiento aditivo / sustractivo
///
/// - **Aditivo**: basta con crear un borde sin tamaño con `classes::Border::default()` para ir
///   añadiendo cada lado lógico con el tamaño deseado usando `ScaleSize::{One..Five}`.
///
/// - **Sustractivo**: se crea un borde con tamaño predefinido, p. ej. usando
///   `classes::Border::new()` o `classes::Border::with(ScaleSize::Two)` y eliminar los lados
///   deseados con `ScaleSize::Zero`.
///
/// - **Anchos diferentes por lado**: usando `ScaleSize::{Zero..Five}` en cada lado deseado.
///
/// # Ejemplos
///
/// **Borde global:**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::with(ScaleSize::Two);
/// assert_eq!(b.to_class(), "border-2");
/// ```
///
/// **Aditivo (solo borde superior):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::default().with_side(Side::Top, ScaleSize::One);
/// assert_eq!(b.to_class(), "border-top-1");
/// ```
///
/// **Sustractivo (borde global menos el superior):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::new().with_side(Side::Top, ScaleSize::Zero);
/// assert_eq!(b.to_class(), "border border-top-0");
/// ```
///
/// **Ancho por lado (lado lógico inicial a 2 y final a 4):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::default()
///     .with_side(Side::Start, ScaleSize::Two)
///     .with_side(Side::End, ScaleSize::Four);
/// assert_eq!(b.to_class(), "border-end-4 border-start-2");
/// ```
///
/// **Combinado (ejemplo completo):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::new()                      // Borde por defecto.
///     .with_side(Side::Top, ScaleSize::Zero)          // Quita borde superior.
///     .with_side(Side::End, ScaleSize::Three)         // Ancho 3 para el lado lógico final.
///     .with_color(BorderColor::Theme(Color::Primary))
///     .with_opacity(Opacity::Half);
///
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
    color  : BorderColor,
    opacity: Opacity,
}

impl Border {
    /// Prepara un borde del tamaño predefinido. Equivale a `border` (ancho por defecto del tema).
    pub fn new() -> Self {
        Self::with(ScaleSize::Auto)
    }

    /// Crea un borde **con un tamaño global** (`size`).
    pub fn with(size: ScaleSize) -> Self {
        Self::default().with_side(Side::All, size)
    }

    // **< Border BUILDER >*************************************************************************

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
        };
        self
    }

    /// Establece el color del borde.
    pub fn with_color(mut self, color: BorderColor) -> Self {
        self.color = color;
        self
    }

    /// Establece la opacidad del borde.
    pub fn with_opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = opacity;
        self
    }

    // **< Border HELPERS >*************************************************************************

    /// Añade las clases de borde a la cadena de clases.
    ///
    /// Concatena, en este orden, las clases para *global*, `top`, `end`, `bottom`, `start`,
    /// *color* y *opacidad*; respetando LTR/RTL y omitiendo las definiciones vacías.
    #[rustfmt::skip]
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        self.all    .push_class(classes, "border");
        self.top    .push_class(classes, "border-top");
        self.end    .push_class(classes, "border-end");
        self.bottom .push_class(classes, "border-bottom");
        self.start  .push_class(classes, "border-start");
        self.color  .push_class(classes);
        self.opacity.push_class(classes, "border");
    }

    /// Devuelve las clases de borde como cadena (`"border-2"`,
    /// `"border border-top-0 border-end-3 border-primary border-opacity-50"`, etc.).
    ///
    /// Si no se define ningún tamaño, color ni opacidad, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut classes = String::new();
        self.push_class(&mut classes);
        classes
    }
}

/// Atajo para crear un [`classes::Border`](crate::theme::classes::Border) a partir de un tamaño
/// [`ScaleSize`] aplicado a todo el borde.
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// // Convertir explícitamente con `From::from`:
/// let b = classes::Border::from(ScaleSize::Two);
/// assert_eq!(b.to_class(), "border-2");
///
/// // Convertir implícitamente con `into()`:
/// let b: classes::Border = ScaleSize::Auto.into();
/// assert_eq!(b.to_class(), "border");
/// ```
impl From<ScaleSize> for Border {
    fn from(size: ScaleSize) -> Self {
        Self::with(size)
    }
}
