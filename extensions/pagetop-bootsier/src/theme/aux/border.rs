use pagetop::prelude::*;

use crate::prelude::*;

use std::fmt;

// **< BorderSize >*********************************************************************************

/// Tamaño (**ancho**) para los bordes ([`Border`]).
///
/// Mapea a `border`, `border-0` y `border-{1..5}`:
///
/// - `None` no añade clase (devuelve `""`).
/// - `Default` genera `border` (borde por defecto del tema).
/// - `Zero` genera `border-0` (sin borde).
/// - `Scale{1..5}` genera `border-{1..5}` (ancho creciente).
#[derive(AutoDefault)]
pub enum BorderSize {
    #[default]
    None,
    Default,
    Zero,
    Scale1,
    Scale2,
    Scale3,
    Scale4,
    Scale5,
}

impl BorderSize {
    #[rustfmt::skip]
    fn to_class(&self, prefix: impl AsRef<str>) -> String {
        match self {
            Self::None    => String::new(),
            Self::Default => String::from(prefix.as_ref()),
            Self::Zero    => join!(prefix, "-0"),
            Self::Scale1  => join!(prefix, "-1"),
            Self::Scale2  => join!(prefix, "-2"),
            Self::Scale3  => join!(prefix, "-3"),
            Self::Scale4  => join!(prefix, "-4"),
            Self::Scale5  => join!(prefix, "-5"),
        }
    }
}

impl fmt::Display for BorderSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class("border"))
    }
}

// **< Border >*************************************************************************************

/// Agrupa propiedades para crear **bordes**.
///
/// Permite:
///
/// - Definir un tamaño **global** para todo el borde (`size`).
/// - Ajustar el tamaño de cada **lado lógico** (`top`, `end`, `bottom`, `start`, **en este orden**,
///   respetando LTR/RTL).
/// - Aplicar un **color** al borde (`BorderColor`).
/// - Aplicar un nivel de **opacidad** (`BorderOpacity`).
///
/// # Comportamiento aditivo / sustractivo
///
/// - **Aditivo**: basta con crear un borde sin tamaño con `Border::new()` para ir añadiendo cada
///   lado lógico con el tamaño deseado usando `BorderSize::Scale{1..5}`.
///
/// - **Sustractivo**: se crea un borde con tamaño predefinido, p. ej. utilizando
///   `Border::with(BorderSize::Scale2)` y eliminar los lados deseados con `BorderSize::Zero`.
///
/// - **Anchos diferentes por lado**: usando `BorderSize::Scale{1..5}` en cada lado deseado.
///
/// # Ejemplos
///
/// **Borde global:**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = Border::with(BorderSize::Scale2);
/// assert_eq!(b.to_string(), "border-2");
/// ```
///
/// **Aditivo (solo borde superior):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = Border::new().with_top(BorderSize::Scale1);
/// assert_eq!(b.to_string(), "border-top-1");
/// ```
///
/// **Sustractivo (borde global menos el superior):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = Border::with(BorderSize::Default).with_top(BorderSize::Zero);
/// assert_eq!(b.to_string(), "border border-top-0");
/// ```
///
/// **Ancho por lado (lado lógico inicial a 2 y final a 4):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = Border::new().with_start(BorderSize::Scale2).with_end(BorderSize::Scale4);
/// assert_eq!(b.to_string(), "border-end-4 border-start-2");
/// ```
///
/// **Combinado (ejemplo completo):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = Border::with(BorderSize::Default)   // Borde global por defecto.
///     .with_top(BorderSize::Zero)             // Quita borde superior.
///     .with_end(BorderSize::Scale3)           // Ancho 3 para el lado lógico final.
///     .with_color(BorderColor::Theme(Color::Primary))
///     .with_opacity(BorderOpacity::Theme(Opacity::Half));
///
/// assert_eq!(b.to_string(), "border border-top-0 border-end-3 border-primary border-opacity-50");
/// ```
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Border {
    size   : BorderSize,
    top    : BorderSize,
    end    : BorderSize,
    bottom : BorderSize,
    start  : BorderSize,
    color  : BorderColor,
    opacity: BorderOpacity,
}

impl Border {
    /// Prepara un borde **sin tamaño global** de partida.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea un borde **con tamaño global** (`size`).
    pub fn with(size: BorderSize) -> Self {
        Self::default().with_size(size)
    }

    // **< Border BUILDER >*************************************************************************

    /// Establece el tamaño global del borde (`border*`).
    pub fn with_size(mut self, size: BorderSize) -> Self {
        self.size = size;
        self
    }

    /// Establece el tamaño del borde superior (`border-top-*`).
    pub fn with_top(mut self, size: BorderSize) -> Self {
        self.top = size;
        self
    }

    /// Establece el tamaño del borde en el lado lógico final (`border-end-*`). Respeta LTR/RTL.
    pub fn with_end(mut self, size: BorderSize) -> Self {
        self.end = size;
        self
    }

    /// Establece el tamaño del borde inferior (`border-bottom-*`).
    pub fn with_bottom(mut self, size: BorderSize) -> Self {
        self.bottom = size;
        self
    }

    /// Establece el tamaño del borde en el lado lógico inicial (`border-start-*`). Respeta LTR/RTL.
    pub fn with_start(mut self, size: BorderSize) -> Self {
        self.start = size;
        self
    }

    /// Establece el **color** del borde.
    pub fn with_color(mut self, color: BorderColor) -> Self {
        self.color = color;
        self
    }

    /// Establece la **opacidad** del borde.
    pub fn with_opacity(mut self, opacity: BorderOpacity) -> Self {
        self.opacity = opacity;
        self
    }
}

impl fmt::Display for Border {
    /// Concatena cada definición en el orden: *global*, `top`, `end`, `bottom`, `start`, *color* y
    /// *opacidad*; respetando LTR/RTL y omitiendo las definiciones vacías.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            join_opt!([
                self.size.to_string(),
                self.top.to_class("border-top"),
                self.end.to_class("border-end"),
                self.bottom.to_class("border-bottom"),
                self.start.to_class("border-start"),
                self.color.to_string(),
                self.opacity.to_string(),
            ]; " ")
            .unwrap_or_default()
        )
    }
}
