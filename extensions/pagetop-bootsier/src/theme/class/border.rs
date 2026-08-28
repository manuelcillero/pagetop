use pagetop::prelude::*;

use crate::theme::{BootsierColors, BoxSide, OpacityLevel, ScaleSize};

// **< BorderColor >********************************************************************************

/// Esquema de color para los bordes ([`Border`]).
///
/// - `Solid(BootsierColors)` y `Subtle(BootsierColors)` usan la paleta de colores temáticos
///   ([`BootsierColors`]).
/// - `Black` y `White` son colores fijos independientes del tema.
/// - `Default` no genera ninguna clase.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum BorderColor {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Genera la clase `border-{color}`.
    Solid(BootsierColors),
    /// Genera la clase `border-{color}-subtle` (un tono suavizado del color).
    Subtle(BootsierColors),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
}

impl BorderColor {
    // Devuelve el sufijo de la clase `border-*`, o `None` si no define ninguna clase.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Default   => None,
            Self::Solid(_)  => Some(""),
            Self::Subtle(_) => Some("-subtle"),
            Self::Black     => Some("-black"),
            Self::White     => Some("-white"),
        }
    }

    /// Añade la clase `border-*` a la cadena de clases.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Solid(c) | Self::Subtle(c) => {
                    classes.push_str("border-");
                    classes.push_str(c.as_str());
                }
                _ => classes.push_str("border"),
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase `border-*` correspondiente al color de borde.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let solid = class::BorderColor::Solid(BootsierColors::Primary).to_class();
    /// assert_eq!(solid, "border-primary");
    ///
    /// let subtle = class::BorderColor::Subtle(BootsierColors::Warning).to_class();
    /// assert_eq!(subtle, "border-warning-subtle");
    ///
    /// let black = class::BorderColor::Black.to_class();
    /// assert_eq!(black, "border-black");
    ///
    /// let none = class::BorderColor::Default.to_class();
    /// assert_eq!(none, "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl From<BootsierColors> for BorderColor {
    /// Convierte un [`BootsierColors`] en [`BorderColor::Solid`].
    ///
    /// Es el atajo habitual para los colores temáticos. Para los demás esquemas (`Subtle`, `Black`,
    /// `White`) sigue usando [`BorderColor`].
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let border: class::BorderColor = BootsierColors::Success.into();
    /// assert_eq!(border.to_class(), "border-success");
    /// ```
    fn from(color: BootsierColors) -> Self {
        Self::Solid(color)
    }
}

impl From<BorderColor> for CowStr {
    /// Permite pasar [`BorderColor`] directamente a [`PropsOp`].
    fn from(val: BorderColor) -> Self {
        val.to_class().into()
    }
}

// **< Border >*************************************************************************************

/// Clases para definir **bordes**.
///
/// Se puede:
///
/// - Iniciar un borde sin tamaño inicial (`Border::default()`).
/// - Crear un borde con tamaño por defecto (`Border::new()`).
/// - Ajustar el tamaño de cada **lado lógico** (`side`, respetando LTR/RTL).
/// - Asignar **un tamaño global** para todo el borde (`size`).
/// - Aplicar un **color** al borde (`BorderColor`).
/// - Aplicar un nivel de **opacidad** (`OpacityLevel`).
///
/// # Comportamiento aditivo / sustractivo
///
/// - **Aditivo**: basta con crear un borde sin tamaño con `class::Border::default()` para ir
///   añadiendo cada lado lógico con el tamaño deseado usando `ScaleSize::{One..Five}`.
///
/// - **Sustractivo**: se crea un borde con tamaño predefinido, por ejemplo usando
///   `class::Border::new()` o `class::Border::with(ScaleSize::Two)` y eliminar los lados deseados
///   con `ScaleSize::Zero`.
///
/// - **Anchos diferentes por lado**: usando `ScaleSize::{Zero..Five}` en cada lado deseado.
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// // Borde global.
/// let b = class::Border::with(ScaleSize::Two);
/// assert_eq!(b.to_class(), "border-2");
///
/// // Aditivo (sólo borde superior):
/// let b = class::Border::default().with_side(BoxSide::Top, ScaleSize::One);
/// assert_eq!(b.to_class(), "border-top-1");
///
/// // Sustractivo (borde global menos el superior):
/// let b = class::Border::new().with_side(BoxSide::Top, ScaleSize::Zero);
/// assert_eq!(b.to_class(), "border border-top-0");
///
/// // Ancho por lado (lado lógico inicial a 2 y final a 4):
/// let b = class::Border::default()
///     .with_side(BoxSide::Start, ScaleSize::Two)
///     .with_side(BoxSide::End, ScaleSize::Four);
/// assert_eq!(b.to_class(), "border-end-4 border-start-2");
///
/// // Combinado (ejemplo completo):
/// let b = class::Border::new()                      // Borde por defecto.
///     .with_side(BoxSide::Top, ScaleSize::Zero)     // Quita borde superior.
///     .with_side(BoxSide::End, ScaleSize::Three)    // Ancho 3 para lado lógico final.
///     .with_color(BootsierColors::Primary)
///     .with_opacity(OpacityLevel::Half);
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
    opacity: OpacityLevel,
}

impl Border {
    /// Prepara un borde del tamaño predefinido. Equivale a `border` (ancho por defecto del tema).
    pub fn new() -> Self {
        Self::with(ScaleSize::Auto)
    }

    /// Define un borde con un tamaño global (`size`) para todos los lados.
    pub fn with(size: ScaleSize) -> Self {
        Self::default().with_side(BoxSide::All, size)
    }

    // **< Border BUILDER >*************************************************************************

    /// Ajusta el tamaño del borde en el lado indicado (ver [`BoxSide`](crate::theme::BoxSide)).
    pub fn with_side(mut self, side: BoxSide, size: ScaleSize) -> Self {
        match side {
            BoxSide::All => self.all = size,
            BoxSide::Top => self.top = size,
            BoxSide::Bottom => self.bottom = size,
            BoxSide::Start => self.start = size,
            BoxSide::End => self.end = size,
            BoxSide::LeftAndRight => {
                self.start = size;
                self.end = size;
            }
            BoxSide::TopAndBottom => {
                self.top = size;
                self.bottom = size;
            }
        }
        self
    }

    /// Establece el color del borde.
    ///
    /// Acepta un tipo convertible en [`BorderColor`]. Un [`BootsierColors`] se convierte
    /// automáticamente en [`BorderColor::Solid`].
    pub fn with_color(mut self, color: impl Into<BorderColor>) -> Self {
        self.color = color.into();
        self
    }

    /// Establece la opacidad del borde.
    pub fn with_opacity(mut self, opacity: OpacityLevel) -> Self {
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
    /// let b = class::Border::from(ScaleSize::Two);
    /// assert_eq!(b.to_class(), "border-2");
    ///
    /// // Convertir implícitamente con `into()`:
    /// let b: class::Border = ScaleSize::Auto.into();
    /// assert_eq!(b.to_class(), "border");
    /// ```
    fn from(size: ScaleSize) -> Self {
        Self::with(size)
    }
}

impl From<Border> for CowStr {
    /// Permite pasar [`Border`] directamente a [`PropsOp`].
    fn from(val: Border) -> Self {
        val.to_class().into()
    }
}
