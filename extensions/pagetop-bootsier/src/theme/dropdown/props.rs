use pagetop::prelude::*;

use crate::prelude::*;

// **< AutoClose >**********************************************************************************

/// Estrategia para el cierre automático de un menú [`Dropdown`].
///
/// Define cuándo se cierra el menú desplegado según la interacción del usuario.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl AutoClose {
    /// Devuelve el valor para `data-bs-auto-close`, o `None` si es el comportamiento por defecto.
    #[rustfmt::skip]
    #[inline]
    pub(crate) const fn as_str(self) -> Option<&'static str> {
        match self {
            Self::Default          => None,
            Self::ClickableInside  => Some("inside"),
            Self::ClickableOutside => Some("outside"),
            Self::ManualClose      => Some("false"),
        }
    }
}

// **< Direction >**********************************************************************************

/// Dirección de despliegue de un menú [`Dropdown`].
///
/// Controla desde qué posición se muestra el menú respecto al botón.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl Direction {
    /// Mapea la dirección teniendo en cuenta si se agrupa con otros menús [`Dropdown`].
    #[rustfmt::skip ]
    #[inline]
    const fn as_str(self, grouped: bool) -> &'static str {
        match self {
            Self::Default if grouped => "",
            Self::Default            => "dropdown",
            Self::Centered           => "dropdown-center",
            Self::Dropup             => "dropup",
            Self::DropupCentered     => "dropup-center",
            Self::Dropend            => "dropend",
            Self::Dropstart          => "dropstart",
        }
    }

    /// Añade la dirección de despliegue a la cadena de clases teniendo en cuenta si se agrupa con
    /// otros menús [`Dropdown`].
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String, grouped: bool) {
        if grouped {
            if !classes.is_empty() {
                classes.push(' ');
            }
            classes.push_str("btn-group");
        }
        let class = self.as_str(grouped);
        if !class.is_empty() {
            if !classes.is_empty() {
                classes.push(' ');
            }
            classes.push_str(class);
        }
    }

    /// Devuelve la clase asociada a la dirección teniendo en cuenta si se agrupa con otros menús
    /// [`Dropdown`], o `""` si no corresponde ninguna.
    #[inline]
    pub(crate) fn class_with(self, grouped: bool) -> String {
        let mut classes = String::new();
        self.push_class(&mut classes, grouped);
        classes
    }
}

// **< MenuAlign >**********************************************************************************

/// Alineación horizontal del menú desplegable [`Dropdown`].
///
/// Permite alinear el menú al inicio o al final del botón (respetando LTR/RTL) y añadirle una
/// alineación diferente a partir de un punto de ruptura ([`BreakPoint`]).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl MenuAlign {
    #[inline]
    fn push_one(classes: &mut String, class: &str) {
        if class.is_empty() {
            return;
        }
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(class);
    }

    /// Añade las clases de alineación a `classes` (sin incluir la base `dropdown-menu`).
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        match self {
            // Alineación por defecto (start), no añade clases extra.
            Self::Start => {}

            // `dropdown-menu-{bp}-start`
            Self::StartAt(bp) => {
                let class = bp.class_with("dropdown-menu", "start");
                Self::push_one(classes, &class);
            }

            // `dropdown-menu-start` + `dropdown-menu-{bp}-end`
            Self::StartAndEnd(bp) => {
                Self::push_one(classes, "dropdown-menu-start");
                let bp_class = bp.class_with("dropdown-menu", "end");
                Self::push_one(classes, &bp_class);
            }

            // `dropdown-menu-end`
            Self::End => {
                Self::push_one(classes, "dropdown-menu-end");
            }

            // `dropdown-menu-{bp}-end`
            Self::EndAt(bp) => {
                let class = bp.class_with("dropdown-menu", "end");
                Self::push_one(classes, &class);
            }

            // `dropdown-menu-end` + `dropdown-menu-{bp}-start`
            Self::EndAndStart(bp) => {
                Self::push_one(classes, "dropdown-menu-end");
                let bp_class = bp.class_with("dropdown-menu", "start");
                Self::push_one(classes, &bp_class);
            }
        }
    }

    /* Devuelve las clases de alineación sin incluir `dropdown-menu` (reservado).
    #[inline]
    pub(crate) fn to_class(self) -> String {
        let mut classes = String::new();
        self.push_class(&mut classes);
        classes
    } */
}

// **< MenuPosition >*******************************************************************************

/// Posición relativa del menú desplegable [`Dropdown`].
///
/// Permite indicar un desplazamiento (*offset*) manual o referenciar al elemento padre para el
/// cálculo de la posición.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl MenuPosition {
    /// Devuelve el valor para `data-bs-offset` o `None` si no aplica.
    #[inline]
    pub(crate) fn data_offset(self) -> Option<String> {
        match self {
            Self::Offset(x, y) => Some(format!("{x},{y}")),
            _ => None,
        }
    }

    /// Devuelve el valor para `data-bs-reference` o `None` si no aplica.
    #[inline]
    pub(crate) fn data_reference(self) -> Option<&'static str> {
        match self {
            Self::Parent => Some("parent"),
            _ => None,
        }
    }
}
