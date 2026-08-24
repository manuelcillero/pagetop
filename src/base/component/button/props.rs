use crate::prelude::*;

use std::fmt;

// **< ButtonStyle >********************************************************************************

/// Estilo visual de un [`Button`](super::Button).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ButtonStyle {
    /// Sin clase de estilo (estilo por defecto del tema).
    #[default]
    None,
    /// Botón sólido: genera la clase `button-{color}`.
    Solid(Intent),
    /// Botón con contorno: genera la clase `button-outline-{color}`.
    Outline(Intent),
    /// Botón tipo enlace: genera la clase `button-link`.
    Link,
}

// **< ButtonKind >*********************************************************************************

/// Comportamiento de un [`Button`](super::Button) al activarse.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ButtonKind {
    /// Envía un formulario al servidor. Es el **tipo por defecto**.
    #[default]
    Submit,
    /// Restablece todos los campos de un formulario a sus valores iniciales.
    Reset,
    /// Botón de propósito general, sin efecto predeterminado. Su comportamiento podría definirse
    /// mediante JavaScript.
    Plain,
}

impl fmt::Display for ButtonKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ButtonKind::Submit => "submit",
            ButtonKind::Reset => "reset",
            ButtonKind::Plain => "button",
        })
    }
}
