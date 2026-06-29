use pagetop::prelude::*;

// **< Backdrop >***********************************************************************************

/// Comportamiento de la capa de fondo (*backdrop*) de un panel
/// [`Offcanvas`](crate::theme::bs::Offcanvas) al deslizarse.
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

impl Backdrop {
    /// Devuelve el valor para `data-bs-backdrop`, o `None` si es el comportamiento por defecto.
    #[rustfmt::skip]
    #[inline]
    pub const fn opt_str(self) -> Option<&'static str> {
        match self {
            Self::Disabled => Some("false"),
            Self::Enabled  => None,
            Self::Static   => Some("static"),
        }
    }
}

// **< BodyScroll >*********************************************************************************

/// Controla si la página principal puede desplazarse al abrir un panel
/// [`Offcanvas`](crate::theme::bs::Offcanvas).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum BodyScroll {
    /// Opción por defecto, la página principal se bloquea centrando la interacción en el panel.
    #[default]
    Disabled,
    /// Permite el desplazamiento de la página principal.
    Enabled,
}

impl BodyScroll {
    /// Devuelve el valor para `data-bs-scroll`, o `None` si es el comportamiento por defecto.
    #[inline]
    pub const fn opt_str(self) -> Option<&'static str> {
        match self {
            Self::Disabled => None,
            Self::Enabled => Some("true"),
        }
    }
}

// **< Placement >**********************************************************************************

/// Posición de aparición de un panel [`Offcanvas`](crate::theme::bs::Offcanvas) al deslizarse.
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
    /// Devuelve la clase base asociada a la posición de aparición del panel.
    #[rustfmt::skip]
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Placement::Start  => "offcanvas-start",
            Placement::End    => "offcanvas-end",
            Placement::Top    => "offcanvas-top",
            Placement::Bottom => "offcanvas-bottom",
        }
    }

    /// Añade la clase asociada a la posición de aparición del panel a la cadena de clases.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(self.as_str());
    }

    /// Devuelve la clase asociada a la posición de aparición del panel.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

// **< Visibility >*********************************************************************************

/// Estado inicial de un panel [`Offcanvas`](crate::theme::bs::Offcanvas).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Visibility {
    /// El panel permanece oculto desde el principio.
    #[default]
    Default,
    /// El panel se muestra abierto al cargar.
    Show,
}

impl Visibility {
    /// Devuelve la clase base asociada al estado inicial del panel.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Visibility::Default => "",
            Visibility::Show => "show",
        }
    }

    /// Añade la clase asociada al estado inicial del panel a la cadena de clases.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        let class = self.as_str();
        if class.is_empty() {
            return;
        }
        if !classes.is_empty() {
            classes.push(' ');
        }
        classes.push_str(class);
    }

    /// Devuelve la clase asociada al estado inicial del panel.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}
