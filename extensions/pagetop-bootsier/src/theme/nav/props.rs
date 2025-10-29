use pagetop::prelude::*;

// **< Kind >***************************************************************************************

/// Define la variante de presentación de un menú [`Nav`](crate::theme::Nav).
#[derive(AutoDefault)]
pub enum Kind {
    /// Estilo por defecto, lista de enlaces flexible y minimalista.
    #[default]
    Default,
    /// Pestañas con borde para cambiar entre secciones.
    Tabs,
    /// Botones con fondo que resaltan el elemento activo.
    Pills,
    /// Variante con subrayado del elemento activo, estética ligera.
    Underline,
}

// **< Layout >*************************************************************************************

/// Distribución y orientación de un menú [`Nav`](crate::theme::Nav).
#[derive(AutoDefault)]
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
