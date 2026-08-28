use pagetop::prelude::*;

// **< Kind >***************************************************************************************

/// Define la variante de presentación de un menú [`Nav`](crate::theme::bs::Nav).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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

impl Kind {
    const TABS: &str = "nav-tabs";
    const PILLS: &str = "nav-pills";
    const UNDERLINE: &str = "nav-underline";

    /// Devuelve la clase base asociada al tipo de menú, o una cadena vacía si no aplica.
    #[rustfmt::skip]
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default   => "",
            Self::Tabs      => Self::TABS,
            Self::Pills     => Self::PILLS,
            Self::Underline => Self::UNDERLINE,
        }
    }

    /// Añade la clase asociada al tipo de menú a la cadena de clases.
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

    /// Devuelve la clase asociada al tipo de menú.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}
