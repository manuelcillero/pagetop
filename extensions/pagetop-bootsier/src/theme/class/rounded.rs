use pagetop::prelude::*;

use crate::theme::token::RoundedRadius;

/// Clases para definir **esquinas redondeadas**.
///
/// Permite:
///
/// - Definir un radio **global para todas las esquinas** (`radius`).
/// - Ajustar el radio asociado a las **esquinas de cada lado lógico** (`top`, `end`, `bottom`,
///   `start`, **en este orden**, respetando LTR/RTL).
/// - Ajustar el radio de las **esquinas concretas** (`top-start`, `top-end`, `bottom-start`,
///   `bottom-end`, **en este orden**, respetando LTR/RTL).
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// // Radio global:
/// let r = class::Rounded::with(token::RoundedRadius::Default);
/// assert_eq!(r.to_class(), "rounded");
///
/// // Sin redondeo:
/// let r = class::Rounded::new();
/// assert_eq!(r.to_class(), "");
///
/// // Radio en las esquinas de un lado lógico:
/// let r = class::Rounded::new().with_end(token::RoundedRadius::Scale2);
/// assert_eq!(r.to_class(), "rounded-end-2");
///
/// // Radio en una esquina concreta:
/// let r = class::Rounded::new().with_top_start(token::RoundedRadius::Scale3);
/// assert_eq!(r.to_class(), "rounded-top-start-3");
///
/// // Combinado (ejemplo completo):
/// let r = class::Rounded::new()
///     .with_top(token::RoundedRadius::Default)          // Añade redondeo arriba.
///     .with_bottom_start(token::RoundedRadius::Scale4)  // Añade una esquina redondeada concreta.
///     .with_bottom_end(token::RoundedRadius::Circle);   // Añade redondeo extremo en otra esquina.
/// assert_eq!(r.to_class(), "rounded-top rounded-bottom-start-4 rounded-bottom-end-circle");
/// ```
#[rustfmt::skip]
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Rounded {
    radius      : RoundedRadius,
    top         : RoundedRadius,
    end         : RoundedRadius,
    bottom      : RoundedRadius,
    start       : RoundedRadius,
    top_start   : RoundedRadius,
    top_end     : RoundedRadius,
    bottom_start: RoundedRadius,
    bottom_end  : RoundedRadius,
}

impl Rounded {
    /// Prepara las esquinas **sin redondeo global** de partida.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea las esquinas **con redondeo global** (`radius`).
    pub fn with(radius: RoundedRadius) -> Self {
        Self::default().with_radius(radius)
    }

    // **< Rounded BUILDER >************************************************************************

    /// Establece el radio global de las esquinas (`rounded*`).
    pub fn with_radius(mut self, radius: RoundedRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Establece el radio en las esquinas del lado superior (`rounded-top-*`).
    pub fn with_top(mut self, radius: RoundedRadius) -> Self {
        self.top = radius;
        self
    }

    /// Establece el radio en las esquinas del lado lógico final (`rounded-end-*`). Respeta LTR/RTL.
    pub fn with_end(mut self, radius: RoundedRadius) -> Self {
        self.end = radius;
        self
    }

    /// Establece el radio en las esquinas del lado inferior (`rounded-bottom-*`).
    pub fn with_bottom(mut self, radius: RoundedRadius) -> Self {
        self.bottom = radius;
        self
    }

    /// Establece el radio en las esquinas del lado lógico inicial (`rounded-start-*`). Respeta
    /// LTR/RTL.
    pub fn with_start(mut self, radius: RoundedRadius) -> Self {
        self.start = radius;
        self
    }

    /// Establece el radio en la esquina superior-inicial (`rounded-top-start-*`). Respeta LTR/RTL.
    pub fn with_top_start(mut self, radius: RoundedRadius) -> Self {
        self.top_start = radius;
        self
    }

    /// Establece el radio en la esquina superior-final (`rounded-top-end-*`). Respeta LTR/RTL.
    pub fn with_top_end(mut self, radius: RoundedRadius) -> Self {
        self.top_end = radius;
        self
    }

    /// Establece el radio en la esquina inferior-inicial (`rounded-bottom-start-*`). Respeta
    /// LTR/RTL.
    pub fn with_bottom_start(mut self, radius: RoundedRadius) -> Self {
        self.bottom_start = radius;
        self
    }

    /// Establece el radio en la esquina inferior-final (`rounded-bottom-end-*`). Respeta LTR/RTL.
    pub fn with_bottom_end(mut self, radius: RoundedRadius) -> Self {
        self.bottom_end = radius;
        self
    }

    // **< Rounded HELPERS >************************************************************************

    /// Concatena, en este orden, las clases para *global*, `top`, `end`, `bottom`, `start`,
    /// `top-start`, `top-end`, `bottom-start` y `bottom-end`; respetando LTR/RTL y omitiendo las
    /// definiciones vacías.
    #[rustfmt::skip]
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        self.radius      .push_to(classes, "");
        self.top         .push_to(classes, "rounded-top");
        self.end         .push_to(classes, "rounded-end");
        self.bottom      .push_to(classes, "rounded-bottom");
        self.start       .push_to(classes, "rounded-start");
        self.top_start   .push_to(classes, "rounded-top-start");
        self.top_end     .push_to(classes, "rounded-top-end");
        self.bottom_start.push_to(classes, "rounded-bottom-start");
        self.bottom_end  .push_to(classes, "rounded-bottom-end");
    }

    /// Devuelve las clases de redondeo como cadena (`"rounded"`,
    /// `"rounded-top rounded-bottom-start-4 rounded-bottom-end-circle"`, etc.).
    ///
    /// Si no se define ningún radio, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut classes = String::new();
        self.push_to(&mut classes);
        classes
    }
}

impl Into<CowStr> for Rounded {
    /// Permite pasar [`Rounded`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}
