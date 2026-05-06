use pagetop::prelude::*;

use crate::theme::attrs::Color;

// **< ButtonAction >*********************************************************************************

/// Comportamiento de un [`Button`](crate::theme::Button) al activarse.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ButtonAction {
    /// Envía un formulario al servidor. Es el **tipo por defecto**.
    #[default]
    Submit,
    /// Restablece todos los campos de un formulario a sus valores iniciales.
    Reset,
    /// Botón de propósito general, sin efecto predeterminado. Su comportamiento podría definirse
    /// mediante JavaScript.
    Plain,
}

impl std::fmt::Display for ButtonAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ButtonAction::Submit => "submit",
            ButtonAction::Reset => "reset",
            ButtonAction::Plain => "button",
        })
    }
}

// **< ButtonColor >********************************************************************************

/// Esquema de color para [`Button`](crate::theme::Button).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ButtonColor {
    /// No define ninguna clase.
    #[default]
    Default,
    /// Genera la clase `btn-{color}` (botón sólido).
    Background(Color),
    /// Genera la clase `btn-outline-{color}` (fondo transparente con contorno coloreado).
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
            Self::Background(c) => {
                classes.push_str(Self::BTN_PREFIX);
                classes.push_str(c.as_str());
            }
            Self::Outline(c) => {
                classes.push_str(Self::BTN_OUTLINE_PREFIX);
                classes.push_str(c.as_str());
            }
            Self::Link => classes.push_str(Self::BTN_LINK),
            Self::Default => unreachable!(),
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
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_class(&mut class);
        class
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
    /// Botón grande.
    Large,
}

impl ButtonSize {
    const BTN_SM: &str = "btn-sm";
    const BTN_LG: &str = "btn-lg";

    /// Añade la clase de tamaño `btn-sm` o `btn-lg` a la cadena de clases.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        let class = match self {
            Self::Default => return,
            Self::Small => Self::BTN_SM,
            Self::Large => Self::BTN_LG,
        };
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(class);
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
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_class(&mut class);
        class
    }
}
