use crate::prelude::*;

// **< Layout >*************************************************************************************

/// Representa las distintas formas de presentación de una barra de navegación
/// [`Navbar`](super::Navbar).
///
/// Sólo recoge las combinaciones de marca y botón de despliegue independientes de cualquier
/// framework CSS. Un tema puede definir su propia variante de disposición (contenido en un panel
/// lateral...) con su propio tipo, sin depender de éste.
#[derive(AutoDefault, Clone, Debug)]
pub enum Layout {
    /// Barra simple, sin marca de identidad y sin botón de despliegue.
    ///
    /// La barra de navegación no se colapsa.
    #[default]
    Simple,

    /// Barra simple, con botón de despliegue y sin marca de identidad.
    SimpleToggle,

    /// Barra simple, con marca de identidad y sin botón de despliegue.
    ///
    /// La barra de navegación no se colapsa.
    SimpleBrandLeft(Embed<Brand>),

    /// Barra con marca de identidad y botón de despliegue, en ese orden.
    BrandLeft(Embed<Brand>),

    /// Barra con botón de despliegue y marca de identidad, en ese orden.
    BrandRight(Embed<Brand>),
}

// **< Position >***********************************************************************************

/// Posición de una barra de navegación [`Navbar`](super::Navbar) en el documento.
///
/// Las variantes `Fixed*` sacan la barra del flujo del documento, por lo que pueden tapar el
/// contenido que queda debajo: hay que reservarle espacio.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Position {
    /// Barra normal, fluye con el documento.
    #[default]
    Static,
    /// Barra fijada en la parte superior, siempre visible.
    FixedTop,
    /// Barra fijada en la parte inferior, siempre visible.
    FixedBottom,
    /// Barra que se fija en la parte superior al hacer *scroll*.
    StickyTop,
    /// Barra que se fija en la parte inferior al hacer *scroll*.
    StickyBottom,
}

impl Position {
    /// Devuelve la clase CSS asociada a la posición, o una cadena vacía si es `Static`.
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
}
