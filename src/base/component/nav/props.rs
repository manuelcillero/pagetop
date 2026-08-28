use crate::prelude::*;

// **< Layout >*************************************************************************************

/// Distribución y orientación de un menú [`Nav`](super::Nav).
///
/// Las variantes son puramente semánticas: cada tema decide en su propio `setup()` qué clases CSS
/// les corresponden, según su propio vocabulario de estilos.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Layout {
    /// Comportamiento por defecto, ancho definido por el contenido y sin alineación forzada.
    #[default]
    Default,
    /// Alinea los elementos al inicio de la fila.
    Start,
    /// Centra horizontalmente los elementos.
    Center,
    /// Alinea los elementos al final de la fila.
    End,
    /// Apila los elementos en columna.
    Vertical,
    /// Los elementos se expanden para rellenar la fila.
    Fill,
    /// Todos los elementos ocupan el mismo ancho rellenando la fila.
    Justified,
}
