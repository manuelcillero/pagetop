use pagetop::prelude::*;

use std::fmt;

// **< RoundedRadius >******************************************************************************

/// Radio para el redondeo de esquinas ([`Rounded`]).
///
/// Mapea a `rounded`, `rounded-0`, `rounded-{1..5}`, `rounded-circle` y `rounded-pill`.
///
/// - `None` no añade clase (devuelve `""`).
/// - `Default` genera `rounded` (radio por defecto del tema).
/// - `Zero` genera `rounded-0` (sin redondeo).
/// - `Scale{1..5}` genera `rounded-{1..5}` (radio creciente).
/// - `Circle` genera `rounded-circle`.
/// - `Pill` genera `rounded-pill`.
#[derive(AutoDefault)]
pub enum RoundedRadius {
    #[default]
    None,
    Default,
    Zero,
    Scale1,
    Scale2,
    Scale3,
    Scale4,
    Scale5,
    Circle,
    Pill,
}

impl RoundedRadius {
    #[rustfmt::skip]
    fn to_class(&self, base: impl AsRef<str>) -> String {
        match self {
            RoundedRadius::None    => String::new(),
            RoundedRadius::Default => String::from(base.as_ref()),
            RoundedRadius::Zero    => join!(base, "-0"),
            RoundedRadius::Scale1  => join!(base, "-1"),
            RoundedRadius::Scale2  => join!(base, "-2"),
            RoundedRadius::Scale3  => join!(base, "-3"),
            RoundedRadius::Scale4  => join!(base, "-4"),
            RoundedRadius::Scale5  => join!(base, "-5"),
            RoundedRadius::Circle  => join!(base, "-circle"),
            RoundedRadius::Pill    => join!(base, "-pill"),
        }
    }
}

impl fmt::Display for RoundedRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class("rounded"))
    }
}

// **< Rounded >************************************************************************************

/// Agrupa propiedades para crear **esquinas redondeadas**.
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
/// **Radio global:**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let r = Rounded::with(RoundedRadius::Default);
/// assert_eq!(r.to_string(), "rounded");
/// ```
///
/// **Sin redondeo:**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let r = Rounded::new();
/// assert_eq!(r.to_string(), "");
/// ```
///
/// **Radio en las esquinas de un lado lógico:**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let r = Rounded::new().with_end(RoundedRadius::Scale2);
/// assert_eq!(r.to_string(), "rounded-end-2");
/// ```
///
/// **Radio en una esquina concreta:**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let r = Rounded::new().with_top_start(RoundedRadius::Scale3);
/// assert_eq!(r.to_string(), "rounded-top-start-3");
/// ```
///
/// **Combinado (ejemplo completo):**
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let r = Rounded::new()
///     .with_top(RoundedRadius::Default)           // Añade redondeo arriba.
///     .with_bottom_start(RoundedRadius::Scale4)   // Añade una esquina redondeada concreta.
///     .with_bottom_end(RoundedRadius::Circle);    // Añade redondeo extremo en otra esquina.
///
/// assert_eq!(r.to_string(), "rounded-top rounded-bottom-start-4 rounded-bottom-end-circle");
/// ```
#[rustfmt::skip]
#[derive(AutoDefault)]
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
}

impl fmt::Display for Rounded {
    /// Concatena cada definición en el orden: *global*, `top`, `end`, `bottom`, `start`,
    /// `top-start`, `top-end`, `bottom-start` y `bottom-end`; respetando LTR/RTL y omitiendo las
    /// definiciones vacías.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            join_opt!([
                self.radius.to_string(),
                self.top.to_class("rounded-top"),
                self.end.to_class("rounded-end"),
                self.bottom.to_class("rounded-bottom"),
                self.start.to_class("rounded-start"),
                self.top_start.to_class("rounded-top-start"),
                self.top_end.to_class("rounded-top-end"),
                self.bottom_start.to_class("rounded-bottom-start"),
                self.bottom_end.to_class("rounded-bottom-end"),
            ]; " ")
            .unwrap_or_default()
        )
    }
}
