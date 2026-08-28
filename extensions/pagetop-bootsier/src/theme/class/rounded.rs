use pagetop::prelude::*;

// **< RoundedRadius >******************************************************************************

/// Radio para el redondeo de esquinas ([`Rounded`]).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum RoundedRadius {
    /// No define ninguna clase.
    #[default]
    None,
    /// Genera `rounded` (radio por defecto del tema).
    Default,
    /// Genera `rounded-0` (sin redondeo).
    Zero,
    /// Genera `rounded-1`.
    Scale1,
    /// Genera `rounded-2`.
    Scale2,
    /// Genera `rounded-3`.
    Scale3,
    /// Genera `rounded-4`.
    Scale4,
    /// Genera `rounded-5`.
    Scale5,
    /// Genera `rounded-circle`.
    Circle,
    /// Genera `rounded-pill`.
    Pill,
}

impl RoundedRadius {
    // Devuelve el sufijo para `*rounded-*`, o `None` si no define ninguna clase, o `""` para el
    // redondeo por defecto.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::None    => None,
            Self::Default => Some(""),
            Self::Zero    => Some("-0"),
            Self::Scale1  => Some("-1"),
            Self::Scale2  => Some("-2"),
            Self::Scale3  => Some("-3"),
            Self::Scale4  => Some("-4"),
            Self::Scale5  => Some("-5"),
            Self::Circle  => Some("-circle"),
            Self::Pill    => Some("-pill"),
        }
    }

    /// Añade el redondeo de esquinas a la cadena de clases usando el prefijo dado (`rounded-top`,
    /// `rounded-bottom-start`, o vacío para `rounded-*`).
    #[inline]
    pub fn push_to(self, classes: &mut String, prefix: &str) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            if prefix.is_empty() {
                classes.push_str("rounded");
            } else {
                classes.push_str(prefix);
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase `rounded-*` para el redondeo de esquinas.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// assert_eq!(class::RoundedRadius::Default.to_class(), "rounded");
    /// assert_eq!(class::RoundedRadius::Zero.to_class(), "rounded-0");
    /// assert_eq!(class::RoundedRadius::Scale3.to_class(), "rounded-3");
    /// assert_eq!(class::RoundedRadius::Circle.to_class(), "rounded-circle");
    /// assert_eq!(class::RoundedRadius::None.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class, "");
        class
    }
}

impl From<RoundedRadius> for CowStr {
    /// Permite pasar [`RoundedRadius`] directamente a [`PropsOp`].
    fn from(val: RoundedRadius) -> Self {
        val.to_class().into()
    }
}

// **< Rounded >************************************************************************************

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
/// # Comportamiento aditivo / sustractivo
///
/// - **Aditivo**: se parte de [`Rounded::default()`] (sin redondeo) y se van añadiendo lados o
///   esquinas concretas con el radio deseado.
///
/// - **Sustractivo**: se parte de [`Rounded::new()`] o [`Rounded::with`] (radio global ya aplicado)
///   y se anulan lados o esquinas concretas con [`RoundedRadius::Zero`].
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// // Radio global por defecto, equivalente a `Rounded::with(RoundedRadius::Default)`:
/// let r = class::Rounded::new();
/// assert_eq!(r.to_class(), "rounded");
///
/// // Radio global explícito:
/// let r = class::Rounded::with(class::RoundedRadius::Scale3);
/// assert_eq!(r.to_class(), "rounded-3");
///
/// // Sin redondeo (comportamiento de `Default`):
/// let r = class::Rounded::default();
/// assert_eq!(r.to_class(), "");
///
/// // Aditivo (radio en las esquinas de un lado lógico):
/// let r = class::Rounded::default().with_end(class::RoundedRadius::Scale2);
/// assert_eq!(r.to_class(), "rounded-end-2");
///
/// // Aditivo (radio en una esquina concreta):
/// let r = class::Rounded::default().with_top_start(class::RoundedRadius::Scale3);
/// assert_eq!(r.to_class(), "rounded-top-start-3");
///
/// // Sustractivo (radio global menos la esquina superior-inicial):
/// let r = class::Rounded::new().with_top_start(class::RoundedRadius::Zero);
/// assert_eq!(r.to_class(), "rounded rounded-top-start-0");
///
/// // Combinado (ejemplo completo):
/// let r = class::Rounded::default()
///     .with_top(class::RoundedRadius::Default)          // Añade redondeo arriba.
///     .with_bottom_start(class::RoundedRadius::Scale4)  // Añade esquina redondeada concreta.
///     .with_bottom_end(class::RoundedRadius::Circle);   // Añade redondeo máximo en otra esquina.
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
    /// Prepara las esquinas con el **radio de redondeo por defecto** (`rounded`), como
    /// [`Self::with`] con [`RoundedRadius::Default`].
    pub fn new() -> Self {
        Self::default().with_radius(RoundedRadius::Default)
    }

    /// Crea las esquinas con un **radio global** explícito (`radius`).
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

impl From<Rounded> for CowStr {
    /// Permite pasar [`Rounded`] directamente a [`PropsOp`].
    fn from(val: Rounded) -> Self {
        val.to_class().into()
    }
}
