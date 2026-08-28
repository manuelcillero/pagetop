use crate::prelude::*;

use std::fmt;

// **< Kind >***************************************************************************************

/// Comportamiento de un [`Button`](super::Button) al activarse.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// Envía un formulario al servidor. Es el **tipo por defecto**.
    #[default]
    Submit,
    /// Restablece todos los campos de un formulario a sus valores iniciales.
    Reset,
    /// Botón de propósito general, sin efecto predeterminado. Su comportamiento podría definirse
    /// mediante JavaScript.
    Plain,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Kind::Submit => "submit",
            Kind::Reset => "reset",
            Kind::Plain => "button",
        })
    }
}

// **< Size >***************************************************************************************

/// Tamaño visual de un [`Button`](super::Button).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Size {
    /// Sin clase de tamaño (tamaño por defecto del tema).
    #[default]
    None,
    /// Botón compacto: genera la clase `button-sm`.
    Small,
    /// Botón grande: genera la clase `button-lg`.
    Large,
}

// **< Style >**************************************************************************************

/// Estilo visual de un [`Button`](super::Button).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Style {
    /// Sin clase de estilo (estilo por defecto del tema).
    #[default]
    None,
    /// Botón sólido: genera la clase `button-{color}`.
    Solid(Intent),
    /// Botón con contorno: genera la clase `button-outline-{color}`.
    Outline(Intent),
    /// Botón tipo enlace: genera la clase `button-link`. Es sólo un estilo visual; el elemento
    /// sigue siendo un `<button>`. Para un enlace de navegación real, usa
    /// [`Button::anchor()`](super::Button::anchor) en su lugar.
    Link,
}
