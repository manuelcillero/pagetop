use pagetop::prelude::*;

/// Radio para el redondeo de esquinas ([`Rounded`](crate::theme::class::Rounded)).
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

    // Añade el redondeo de esquinas a la cadena de clases usando el prefijo dado (`rounded-top`,
    // `rounded-bottom-start`, o vacío para `rounded-*`).
    #[inline]
    pub(crate) fn push_to(self, classes: &mut String, prefix: &str) {
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
    /// assert_eq!(token::RoundedRadius::Default.to_class(), "rounded");
    /// assert_eq!(token::RoundedRadius::Zero.to_class(), "rounded-0");
    /// assert_eq!(token::RoundedRadius::Scale3.to_class(), "rounded-3");
    /// assert_eq!(token::RoundedRadius::Circle.to_class(), "rounded-circle");
    /// assert_eq!(token::RoundedRadius::None.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class, "");
        class
    }
}
