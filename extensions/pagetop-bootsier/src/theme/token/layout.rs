use pagetop::prelude::*;

// **< ScaleSize >**********************************************************************************

/// Escala discreta de tamaños para clases utilitarias.
///
/// Se usa como parámetro de tamaño para las clases de [`Border`], [`Margin`] y [`Padding`]. La
/// variante `Auto` no aplica en `Padding`.
///
/// [`Border`]: crate::theme::class::Border
/// [`Margin`]: crate::theme::class::Margin
/// [`Padding`]: crate::theme::class::Padding
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ScaleSize {
    /// Sin tamaño (no define ninguna clase).
    #[default]
    None,
    /// Tamaño automático.
    Auto,
    /// Escala cero.
    Zero,
    /// Escala uno.
    One,
    /// Escala dos.
    Two,
    /// Escala tres.
    Three,
    /// Escala cuatro.
    Four,
    /// Escala cinco.
    Five,
}

impl ScaleSize {
    // Devuelve el sufijo de tamaño (`"-0"`, `"-1"`, etc.), o `None` si no define ninguna clase.
    // `Auto` devuelve `Some("")` para que `push_to` emita sólo el prefijo, sin sufijo de tamaño.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::None  => None,
            Self::Auto  => Some(""),
            Self::Zero  => Some("-0"),
            Self::One   => Some("-1"),
            Self::Two   => Some("-2"),
            Self::Three => Some("-3"),
            Self::Four  => Some("-4"),
            Self::Five  => Some("-5"),
        }
    }

    // Añade el tamaño a la cadena de clases usando el prefijo dado.
    #[inline]
    pub(crate) fn push_to(self, classes: &mut String, prefix: &str) {
        if !prefix.is_empty() {
            if let Some(suffix) = self.suffix() {
                if !classes.is_empty() {
                    classes.push(' ');
                }
                classes.push_str(prefix);
                classes.push_str(suffix);
            }
        }
    }
}

// **< BoxSide >************************************************************************************

/// Lados sobre los que aplicar una clase utilitaria (respetando LTR/RTL).
///
/// Se usa como selector de lado para las clases de [`Border`], [`Margin`] y [`Padding`].
///
/// [`Border`]: crate::theme::class::Border
/// [`Margin`]: crate::theme::class::Margin
/// [`Padding`]: crate::theme::class::Padding
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum BoxSide {
    /// Todos los lados.
    #[default]
    All,
    /// Lado superior.
    Top,
    /// Lado inferior.
    Bottom,
    /// Lado lógico de inicio (respetando RTL).
    Start,
    /// Lado lógico de fin (respetando RTL).
    End,
    /// Lados lógicos laterales (abreviatura *x*).
    LeftAndRight,
    /// Lados superior e inferior (abreviatura *y*).
    TopAndBottom,
}
