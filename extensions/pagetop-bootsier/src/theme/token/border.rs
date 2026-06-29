use pagetop::prelude::*;

use crate::theme::token::Color;

/// Esquema de color para los bordes ([`Border`](crate::theme::class::Border)).
///
/// - `Theme(Color)` y `Subtle(Color)` usan la paleta de colores temáticos ([`Color`]).
/// - `Black` y `White` son colores fijos independientes del tema.
/// - `Default` no genera ninguna clase.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ColorBorder {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Genera la clase `border-{color}`.
    Theme(Color),
    /// Genera la clase `border-{color}-subtle` (un tono suavizado del color).
    Subtle(Color),
    /// Color negro.
    Black,
    /// Color blanco.
    White,
}

impl ColorBorder {
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
    pub(crate) fn push_to(self, classes: &mut String) {
        if let Some(suffix) = self.suffix() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            match self {
                Self::Theme(c) | Self::Subtle(c) => {
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
    /// let theme = token::ColorBorder::Theme(token::Color::Primary).to_class();
    /// assert_eq!(theme, "border-primary");
    ///
    /// let subtle = token::ColorBorder::Subtle(token::Color::Warning).to_class();
    /// assert_eq!(subtle, "border-warning-subtle");
    ///
    /// let black = token::ColorBorder::Black.to_class();
    /// assert_eq!(black, "border-black");
    ///
    /// let none = token::ColorBorder::Default.to_class();
    /// assert_eq!(none, "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl From<Color> for ColorBorder {
    /// Convierte un [`Color`] en [`ColorBorder::Theme`].
    ///
    /// Permite pasar un [`Color`] directamente donde se espera un [`ColorBorder`], sin necesidad
    /// de envolver el valor en [`ColorBorder::Theme`]. Es el atajo habitual para los colores
    /// temáticos.
    ///
    /// Para los demás esquemas (`Subtle`, `Black`, `White`) se sigue usando [`ColorBorder`]
    /// directamente.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::theme::*;
    /// let border: token::ColorBorder = token::Color::Success.into();
    /// assert_eq!(border.to_class(), "border-success");
    /// ```
    fn from(color: Color) -> Self {
        Self::Theme(color)
    }
}
