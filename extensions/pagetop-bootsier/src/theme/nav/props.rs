use pagetop::prelude::*;

// **< Kind >***************************************************************************************

/// Define la variante de presentación de un menú [`Nav`](crate::theme::Nav).
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

    // Devuelve la clase base asociada al tipo de menú, o una cadena vacía si no aplica.
    #[rustfmt::skip]
    #[inline]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Default   => "",
            Self::Tabs      => Self::TABS,
            Self::Pills     => Self::PILLS,
            Self::Underline => Self::UNDERLINE,
        }
    }

    // Añade la clase asociada al tipo de menú a la cadena de clases.
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

    /* Devuelve la clase asociada al tipo de menú, o una cadena vacía si no aplica (reservado).
    #[inline]
    pub(crate) fn to_class(self) -> String {
        self.as_str().to_owned()
    } */
}

// **< Layout >*************************************************************************************

/// Distribución y orientación de un menú [`Nav`](crate::theme::Nav).
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

impl Layout {
    const START: &str = "justify-content-start";
    const CENTER: &str = "justify-content-center";
    const END: &str = "justify-content-end";
    const VERTICAL: &str = "flex-column";
    const FILL: &str = "nav-fill";
    const JUSTIFIED: &str = "nav-justified";

    // Devuelve la clase base asociada a la distribución y orientación del menú.
    #[rustfmt::skip]
    #[inline]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Default   => "",
            Self::Start     => Self::START,
            Self::Center    => Self::CENTER,
            Self::End       => Self::END,
            Self::Vertical  => Self::VERTICAL,
            Self::Fill      => Self::FILL,
            Self::Justified => Self::JUSTIFIED,
        }
    }

    // Añade la clase asociada a la distribución y orientación del menú a la cadena de clases.
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

    /* Devuelve la clase asociada a la distribución y orientación del menú, o una cadena vacía si no
    // aplica (reservado).
    #[inline]
    pub(crate) fn to_class(self) -> String {
        self.as_str().to_owned()
    } */
}
