use pagetop::prelude::*;

use crate::theme::aux::{BorderColor, BorderSize, Opacity};

use std::fmt;

/// Clases para crear **bordes**.
///
/// Permite:
///
/// - Definir un tamaño **global** para todo el borde (`size`).
/// - Ajustar el tamaño de cada **lado lógico** (`top`, `end`, `bottom`, `start`, **en este orden**,
///   respetando LTR/RTL).
/// - Aplicar un **color** al borde (`BorderColor`).
/// - Aplicar un nivel de **opacidad** (`Opacity`).
///
/// # Comportamiento aditivo / sustractivo
///
/// - **Aditivo**: basta con crear un borde sin tamaño con `classes::Border::new()` para ir
///   añadiendo cada lado lógico con el tamaño deseado usando `BorderSize::Scale{1..5}`.
///
/// - **Sustractivo**: se crea un borde con tamaño predefinido, p. ej. utilizando
///   `classes::Border::with(BorderSize::Scale2)` y eliminar los lados deseados con `BorderSize::Zero`.
///
/// - **Anchos diferentes por lado**: usando `BorderSize::Scale{1..5}` en cada lado deseado.
///
/// # Ejemplos
///
/// **Borde global:**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::with(BorderSize::Scale2);
/// assert_eq!(b.to_string(), "border-2");
/// ```
///
/// **Aditivo (solo borde superior):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::new().with_top(BorderSize::Scale1);
/// assert_eq!(b.to_string(), "border-top-1");
/// ```
///
/// **Sustractivo (borde global menos el superior):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::with(BorderSize::Default).with_top(BorderSize::Zero);
/// assert_eq!(b.to_string(), "border border-top-0");
/// ```
///
/// **Ancho por lado (lado lógico inicial a 2 y final a 4):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::new().with_start(BorderSize::Scale2).with_end(BorderSize::Scale4);
/// assert_eq!(b.to_string(), "border-end-4 border-start-2");
/// ```
///
/// **Combinado (ejemplo completo):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let b = classes::Border::with(BorderSize::Default)   // Borde global por defecto.
///     .with_top(BorderSize::Zero)                  // Quita borde superior.
///     .with_end(BorderSize::Scale3)                // Ancho 3 para el lado lógico final.
///     .with_color(BorderColor::Theme(Color::Primary))
///     .with_opacity(Opacity::Half);
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
    opacity: Opacity,
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
}

impl fmt::Display for Border {
    /// Concatena, en este orden, las clases para *global*, `top`, `end`, `bottom`, `start`, *color*
    /// y *opacidad*; respetando LTR/RTL y omitiendo las definiciones vacías.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            [
                self.size.to_string(),
                self.top.to_class("border-top"),
                self.end.to_class("border-end"),
                self.bottom.to_class("border-bottom"),
                self.start.to_class("border-start"),
                self.color.to_string(),
                self.opacity.to_class("border"),
            ]
            .join_classes()
        )
    }
}
