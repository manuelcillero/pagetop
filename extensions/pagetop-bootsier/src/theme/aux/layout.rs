use pagetop::prelude::*;

// **< ScaleSize >**********************************************************************************

/// Escala discreta de tamaños para definir clases utilitarias.
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
    /// Devuelve el sufijo para el tamaño (`"-0"`, `"-1"`, etc.), o `None` si no define ninguna
    /// clase, o `""` para el tamaño automático.
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

    /// Añade el tamaño a la cadena de clases usando el prefijo dado.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String, prefix: &str) {
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

    /* Devuelve la clase del tamaño para el prefijo, o una cadena vacía si no aplica (reservado).
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(ScaleSize::Auto.class_with("border"), "border");
    /// assert_eq!(ScaleSize::Zero.class_with("m"), "m-0");
    /// assert_eq!(ScaleSize::Three.class_with("p"), "p-3");
    /// assert_eq!(ScaleSize::None.class_with("border"), "");
    /// ```
    #[doc(hidden)]
    pub fn class_with(self, prefix: &str) -> String {
        if !prefix.is_empty() {
            if let Some(suffix) = self.suffix() {
                let mut class = String::with_capacity(prefix.len() + suffix.len());
                class.push_str(prefix);
                class.push_str(suffix);
                return class;
            }
        }
        String::new()
    } */
}

// **< Side >***************************************************************************************

/// Lados sobre los que aplicar una clase utilitaria (respetando LTR/RTL).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Side {
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
