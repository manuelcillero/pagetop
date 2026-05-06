use pagetop::prelude::*;

/// Radio para el redondeo de esquinas ([`classes::Rounded`](crate::theme::classes::Rounded)).
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
    const ROUNDED: &str = "rounded";

    /// Devuelve el sufijo para `*rounded-*`, o `None` si no define ninguna clase, o `""` para el
    /// redondeo por defecto.
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
    pub(crate) fn push_class(self, classes: &mut String, prefix: &str) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            if prefix.is_empty() {
                classes.push_str(Self::ROUNDED);
            } else {
                classes.push_str(prefix);
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase para el redondeo de esquinas con el prefijo dado (`rounded-top`,
    /// `rounded-bottom-start`, o vacío para `rounded-*`).
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(RoundedRadius::Scale2.class_with(""), "rounded-2");
    /// assert_eq!(RoundedRadius::Zero.class_with("rounded-top"), "rounded-top-0");
    /// assert_eq!(RoundedRadius::Scale3.class_with("rounded-top-end"), "rounded-top-end-3");
    /// assert_eq!(RoundedRadius::Circle.class_with(""), "rounded-circle");
    /// assert_eq!(RoundedRadius::None.class_with("rounded-bottom-start"), "");
    /// ```
    #[doc(hidden)]
    pub fn class_with(self, prefix: &str) -> String {
        if let Some(suffix) = self.suffix() {
            let base_len = if prefix.is_empty() {
                Self::ROUNDED.len()
            } else {
                prefix.len()
            };
            let mut class = String::with_capacity(base_len + suffix.len());
            if prefix.is_empty() {
                class.push_str(Self::ROUNDED);
            } else {
                class.push_str(prefix);
            }
            class.push_str(suffix);
            return class;
        }
        String::new()
    }

    /// Devuelve la clase `rounded-*` para el redondeo de esquinas.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(RoundedRadius::Default.to_class(), "rounded");
    /// assert_eq!(RoundedRadius::Zero.to_class(), "rounded-0");
    /// assert_eq!(RoundedRadius::Scale3.to_class(), "rounded-3");
    /// assert_eq!(RoundedRadius::Circle.to_class(), "rounded-circle");
    /// assert_eq!(RoundedRadius::None.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        self.class_with("")
    }
}
