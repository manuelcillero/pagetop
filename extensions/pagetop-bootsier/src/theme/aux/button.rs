use pagetop::prelude::*;

use crate::theme::aux::Color;

// **< ButtonColor >********************************************************************************

/// Variantes de color `btn-*` para botones.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ButtonColor {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Genera internamente clases `btn-{color}` (botón relleno).
    Background(Color),
    /// Genera `btn-outline-{color}` (fondo transparente y contorno con borde).
    Outline(Color),
    /// Aplica estilo de los enlaces (`btn-link`), sin caja ni fondo, heredando el color de texto.
    Link,
}

impl ButtonColor {
    const BTN_PREFIX: &str = "btn-";
    const BTN_OUTLINE_PREFIX: &str = "btn-outline-";
    const BTN_LINK: &str = "btn-link";

    /// Añade la clase `btn-*` a la cadena de clases.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        if let Self::Default = self {
            return;
        }
        if !classes.is_empty() {
            classes.push(' ');
        }
        match self {
            Self::Default => unreachable!(),
            Self::Background(c) => {
                classes.push_str(Self::BTN_PREFIX);
                classes.push_str(c.as_str());
            }
            Self::Outline(c) => {
                classes.push_str(Self::BTN_OUTLINE_PREFIX);
                classes.push_str(c.as_str());
            }
            Self::Link => {
                classes.push_str(Self::BTN_LINK);
            }
        }
    }

    /// Devuelve la clase `btn-*` correspondiente al color del botón.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(
    ///     ButtonColor::Background(Color::Primary).to_class(),
    ///     "btn-primary"
    /// );
    /// assert_eq!(
    ///     ButtonColor::Outline(Color::Danger).to_class(),
    ///     "btn-outline-danger"
    /// );
    /// assert_eq!(ButtonColor::Link.to_class(), "btn-link");
    /// assert_eq!(ButtonColor::Default.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        match self {
            Self::Default => String::new(),
            Self::Background(c) => {
                let color = c.as_str();
                let mut class = String::with_capacity(Self::BTN_PREFIX.len() + color.len());
                class.push_str(Self::BTN_PREFIX);
                class.push_str(color);
                class
            }
            Self::Outline(c) => {
                let color = c.as_str();
                let mut class = String::with_capacity(Self::BTN_OUTLINE_PREFIX.len() + color.len());
                class.push_str(Self::BTN_OUTLINE_PREFIX);
                class.push_str(color);
                class
            }
            Self::Link => Self::BTN_LINK.to_string(),
        }
    }
}

// **< ButtonSize >*********************************************************************************

/// Tamaño visual de un botón.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    /// Tamaño por defecto del tema (no añade clase).
    #[default]
    Default,
    /// Botón compacto.
    Small,
    /// Botón destacado/grande.
    Large,
}

impl ButtonSize {
    const BTN_SM: &str = "btn-sm";
    const BTN_LG: &str = "btn-lg";

    /// Añade la clase de tamaño `btn-sm` o `btn-lg` a la cadena de clases.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        if let Self::Default = self {
            return;
        }
        if !classes.is_empty() {
            classes.push(' ');
        }
        match self {
            Self::Default => unreachable!(),
            Self::Small => classes.push_str(Self::BTN_SM),
            Self::Large => classes.push_str(Self::BTN_LG),
        }
    }

    /// Devuelve la clase `btn-sm` o `btn-lg` correspondiente al tamaño del botón.
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// assert_eq!(ButtonSize::Small.to_class(), "btn-sm");
    /// assert_eq!(ButtonSize::Large.to_class(), "btn-lg");
    /// assert_eq!(ButtonSize::Default.to_class(), "");
    /// ```
    #[inline]
    pub fn to_class(self) -> String {
        match self {
            Self::Default => String::new(),
            Self::Small => Self::BTN_SM.to_string(),
            Self::Large => Self::BTN_LG.to_string(),
        }
    }
}
