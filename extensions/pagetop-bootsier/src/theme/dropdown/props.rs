use pagetop::prelude::*;

use crate::prelude::*;

// **< AutoClose >**********************************************************************************

/// Estrategia para el cierre automático de un menú [`Dropdown`].
///
/// Define cuándo se cierra el menú desplegado según la interacción del usuario.
#[derive(AutoDefault)]
pub enum AutoClose {
    /// Comportamiento por defecto, se cierra con clics dentro y fuera del menú, o pulsando `Esc`.
    #[default]
    Default,
    /// Sólo se cierra con clics dentro del menú.
    ClickableInside,
    /// Sólo se cierra con clics fuera del menú.
    ClickableOutside,
    /// Cierre manual, no se cierra con clics; sólo al pulsar nuevamente el botón del menú
    /// (*toggle*), o pulsando `Esc`.
    ManualClose,
}

// **< Direction >**********************************************************************************

/// Dirección de despliegue de un menú [`Dropdown`].
///
/// Controla desde qué posición se muestra el menú respecto al botón.
#[derive(AutoDefault)]
pub enum Direction {
    /// Comportamiento por defecto (despliega el menú hacia abajo desde la posición inicial,
    /// respetando LTR/RTL).
    #[default]
    Default,
    /// Centra horizontalmente el menú respecto al botón.
    Centered,
    /// Despliega el menú hacia arriba.
    Dropup,
    /// Despliega el menú hacia arriba y centrado.
    DropupCentered,
    /// Despliega el menú desde el lateral final, respetando LTR/RTL.
    Dropend,
    /// Despliega el menú desde el lateral inicial, respetando LTR/RTL.
    Dropstart,
}

// **< MenuAlign >**********************************************************************************

/// Alineación horizontal del menú desplegable [`Dropdown`].
///
/// Permite alinear el menú al inicio o al final del botón (respetando LTR/RTL) y añadirle una
/// alineación diferente a partir de un punto de ruptura ([`BreakPoint`]).
#[derive(AutoDefault)]
pub enum MenuAlign {
    /// Alineación al inicio (comportamiento por defecto).
    #[default]
    Start,
    /// Alineación al inicio a partir del punto de ruptura indicado.
    StartAt(BreakPoint),
    /// Alineación al inicio por defecto, y al final a partir de un punto de ruptura válido.
    StartAndEnd(BreakPoint),
    /// Alineación al final.
    End,
    /// Alineación al final a partir del punto de ruptura indicado.
    EndAt(BreakPoint),
    /// Alineación al final por defecto, y al inicio a partir de un punto de ruptura válido.
    EndAndStart(BreakPoint),
}

// **< MenuPosition >*******************************************************************************

/// Posición relativa del menú desplegable [`Dropdown`].
///
/// Permite indicar un desplazamiento (*offset*) manual o referenciar al elemento padre para el
/// cálculo de la posición.
#[derive(AutoDefault)]
pub enum MenuPosition {
    /// Posicionamiento automático por defecto.
    #[default]
    Default,
    /// Desplazamiento manual en píxeles `(x, y)` aplicado al menú. Se admiten valores negativos.
    Offset(i8, i8),
    /// Posiciona el menú tomando como referencia el botón padre. Especialmente útil cuando
    /// [`button_split()`](crate::theme::Dropdown::button_split) es `true`.
    Parent,
}
