use pagetop::prelude::*;

use std::fmt;

/// Radio para el redondeo de esquinas ([`classes::Rounded`](crate::theme::classes::Rounded)).
///
/// Mapea a `rounded`, `rounded-0`, `rounded-{1..5}`, `rounded-circle` y `rounded-pill`.
///
/// - `None` no añade ninguna clase.
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
    pub(crate) fn to_class(&self, prefix: impl AsRef<str>) -> String {
        match self {
            RoundedRadius::None    => String::new(),
            RoundedRadius::Default => String::from(prefix.as_ref()),
            RoundedRadius::Zero    => join!(prefix, "-0"),
            RoundedRadius::Scale1  => join!(prefix, "-1"),
            RoundedRadius::Scale2  => join!(prefix, "-2"),
            RoundedRadius::Scale3  => join!(prefix, "-3"),
            RoundedRadius::Scale4  => join!(prefix, "-4"),
            RoundedRadius::Scale5  => join!(prefix, "-5"),
            RoundedRadius::Circle  => join!(prefix, "-circle"),
            RoundedRadius::Pill    => join!(prefix, "-pill"),
        }
    }
}

impl fmt::Display for RoundedRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class("rounded"))
    }
}
