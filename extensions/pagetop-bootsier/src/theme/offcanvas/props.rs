use pagetop::prelude::*;

// **< Backdrop >***********************************************************************************

/// Comportamiento de la capa de fondo (*backdrop*) de un panel
/// [`Offcanvas`](crate::theme::Offcanvas) al deslizarse.
#[derive(AutoDefault)]
pub enum Backdrop {
    /// Sin capa de fondo, la página principal permanece visible e interactiva.
    Disabled,
    /// Opción por defecto, se oscurece el fondo; un clic fuera del panel suele cerrarlo.
    #[default]
    Enabled,
    /// Muestra la capa de fondo pero no se cierra al hacer clic fuera del panel. Útil si se
    /// requiere completar una acción antes de salir.
    Static,
}

// **< BodyScroll >*********************************************************************************

/// Controla si la página principal puede desplazarse al abrir un panel
/// [`Offcanvas`](crate::theme::Offcanvas).
#[derive(AutoDefault)]
pub enum BodyScroll {
    /// Opción por defecto, la página principal se bloquea centrando la interacción en el panel.
    #[default]
    Disabled,
    /// Permite el desplazamiento de la página principal.
    Enabled,
}

// **< Placement >**********************************************************************************

/// Posición de aparición de un panel [`Offcanvas`](crate::theme::Offcanvas) al deslizarse.
///
/// Define desde qué borde de la ventana entra y se ancla el panel.
#[derive(AutoDefault)]
pub enum Placement {
    /// Opción por defecto, desde el borde inicial según dirección de lectura (respetando LTR/RTL).
    #[default]
    Start,
    /// Desde el borde final según dirección de lectura (respetando LTR/RTL).
    End,
    /// Desde la parte superior.
    Top,
    /// Desde la parte inferior.
    Bottom,
}

// **< Visibility >*********************************************************************************

/// Estado inicial de un panel [`Offcanvas`](crate::theme::Offcanvas).
#[derive(AutoDefault)]
pub enum Visibility {
    /// El panel permanece oculto desde el principio.
    #[default]
    Default,
    /// El panel se muestra abierto al cargar.
    Show,
}
