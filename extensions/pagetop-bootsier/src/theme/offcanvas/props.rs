use pagetop::prelude::*;

// **< Backdrop >***********************************************************************************

/// Comportamiento de la capa de fondo (*backdrop*) de un panel
/// [`Offcanvas`](crate::theme::Offcanvas) al deslizarse.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl Placement {
    // Devuelve la clase base asociada a la posición de aparición del panel.
    #[rustfmt::skip]
    #[inline]
    const fn as_str(self) -> &'static str {
        match self {
            Placement::Start  => "offcanvas-start",
            Placement::End    => "offcanvas-end",
            Placement::Top    => "offcanvas-top",
            Placement::Bottom => "offcanvas-bottom",
        }
    }

    // Añade la clase asociada a la posición de aparición del panel a la cadena de clases.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(self.as_str());
    }

    /* Devuelve la clase asociada a la posición de aparición del panel (reservado).
    #[inline]
    pub(crate) fn to_class(self) -> String {
        self.as_str().to_owned()
    } */
}

// **< Visibility >*********************************************************************************

/// Estado inicial de un panel [`Offcanvas`](crate::theme::Offcanvas).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Visibility {
    /// El panel permanece oculto desde el principio.
    #[default]
    Default,
    /// El panel se muestra abierto al cargar.
    Show,
}

impl Visibility {
    // Devuelve la clase base asociada al estado inicial del panel.
    #[inline]
    const fn as_str(self) -> &'static str {
        match self {
            Visibility::Default => "",
            Visibility::Show => "show",
        }
    }

    // Añade la clase asociada al estado inicial del panel a la cadena de clases.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        let class = self.as_str();
        if class.is_empty() {
            return;
        }
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(class);
    }

    /* Devuelve la clase asociada al estado inicial, o una cadena vacía si no aplica (reservado).
    #[inline]
    pub(crate) fn to_class(self) -> String {
        self.as_str().to_owned()
    } */
}
