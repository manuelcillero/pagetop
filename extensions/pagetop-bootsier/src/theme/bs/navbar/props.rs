use pagetop::prelude::*;

use crate::theme::*;

// **< Layout >*************************************************************************************

/// Representa los diferentes tipos de presentación de una barra de navegación
/// [`Navbar`](crate::theme::bs::Navbar).
#[derive(AutoDefault, Clone, Debug)]
pub enum Layout {
    /// Barra simple, sin marca de identidad y sin botón de despliegue.
    ///
    /// La barra de navegación no se colapsa.
    #[default]
    Simple,

    /// Barra simple, con botón de despliegue a la izquierda y sin marca de identidad.
    SimpleToggle,

    /// Barra simple, con marca de identidad a la izquierda y sin botón de despliegue.
    ///
    /// La barra de navegación no se colapsa.
    SimpleBrandLeft(Embed<Brand>),

    /// Barra con marca de identidad a la izquierda y botón de despliegue a la derecha.
    BrandLeft(Embed<Brand>),

    /// Barra con botón de despliegue a la izquierda y marca de identidad a la derecha.
    BrandRight(Embed<Brand>),

    /// Contenido en [`Offcanvas`](crate::theme::bs::Offcanvas), con botón de despliegue a la
    /// izquierda y sin marca de identidad.
    Offcanvas(Embed<bs::Offcanvas>),

    /// Contenido en [`Offcanvas`](crate::theme::bs::Offcanvas), con marca de identidad a la
    /// izquierda y botón de despliegue a la derecha.
    OffcanvasBrandLeft(Embed<Brand>, Embed<bs::Offcanvas>),

    /// Contenido en [`Offcanvas`](crate::theme::bs::Offcanvas), con botón de despliegue a la
    /// izquierda y marca de identidad a la derecha.
    OffcanvasBrandRight(Embed<Brand>, Embed<bs::Offcanvas>),
}

// **< Position >***********************************************************************************

/// Posición global de una barra de navegación [`Navbar`](crate::theme::bs::Navbar) en el documento.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Position {
    /// Barra normal, fluye con el documento.
    #[default]
    Static,
    /// Barra fijada en la parte superior, siempre visible.
    ///
    /// Puede ser necesario reservar espacio en la parte superior del contenido que fluye debajo
    /// para evitar que quede oculto por la barra.
    FixedTop,
    /// Barra fijada en la parte inferior, siempre visible.
    ///
    /// Puede ser necesario reservar espacio en la parte inferior del contenido que fluye debajo
    /// para evitar que quede oculto por la barra.
    FixedBottom,
    /// La barra de navegación se fija en la parte superior al hacer *scroll*.
    StickyTop,
    /// La barra de navegación se fija en la parte inferior al hacer *scroll*.
    StickyBottom,
}

impl Position {
    /// Devuelve la clase base asociada a la posición de la barra de navegación.
    #[rustfmt::skip]
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static       => "",
            Self::FixedTop     => "fixed-top",
            Self::FixedBottom  => "fixed-bottom",
            Self::StickyTop    => "sticky-top",
            Self::StickyBottom => "sticky-bottom",
        }
    }

    /// Añade la clase asociada a la posición de la barra de navegación a la cadena de clases.
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

    /// Devuelve la clase asociada a la posición de la barra de navegación.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}
