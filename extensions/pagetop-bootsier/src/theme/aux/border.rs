use pagetop::prelude::*;

use crate::theme::aux::Color;

/// Colores `border-*` para los bordes ([`classes::Border`](crate::theme::classes::Border)).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum BorderColor {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Genera internamente clases `border-{color}`.
    Theme(Color),
    /// Genera internamente clases `border-{color}-subtle` (un tono suavizado del color).
    Subtle(Color),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
}

impl BorderColor {
    const BORDER: &str = "border";
    const BORDER_PREFIX: &str = "border-";

    // Devuelve el sufijo de la clase `border-*`, o `None` si no define ninguna clase.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Default   => None,
            Self::Theme(_)  => Some(""),
            Self::Subtle(_) => Some("-subtle"),
            Self::Black     => Some("-black"),
            Self::White     => Some("-white"),
        }
    }

    // Añade la clase `border-*` a la cadena de clases.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Theme(c) | Self::Subtle(c) => {
                    classes.push_str(Self::BORDER_PREFIX);
                    classes.push_str(c.as_str());
                }
                _ => classes.push_str(Self::BORDER),
            }
            classes.push_str(suffix);
        }
    }

    /// Devuelve la clase `border-*` correspondiente al color de borde.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(BorderColor::Theme(Color::Primary).to_class(), "border-primary");
    /// assert_eq!(BorderColor::Subtle(Color::Warning).to_class(), "border-warning-subtle");
    /// assert_eq!(BorderColor::Black.to_class(), "border-black");
    /// assert_eq!(BorderColor::Default.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        if let Some(suffix) = self.suffix() {
            let base_len = match self {
                Self::Theme(c) | Self::Subtle(c) => Self::BORDER_PREFIX.len() + c.as_str().len(),
                _ => Self::BORDER.len(),
            };
            let mut class = String::with_capacity(base_len + suffix.len());
            match self {
                Self::Theme(c) | Self::Subtle(c) => {
                    class.push_str(Self::BORDER_PREFIX);
                    class.push_str(c.as_str());
                }
                _ => class.push_str(Self::BORDER),
            }
            class.push_str(suffix);
            return class;
        }
        String::new()
    }
}
