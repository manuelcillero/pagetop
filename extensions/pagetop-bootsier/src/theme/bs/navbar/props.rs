use pagetop::prelude::*;

use crate::theme::*;

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

    /// Contenido en un [`Panel`](crate::theme::bs::navbar::Panel) lateral, con botón de despliegue
    /// a la izquierda y sin marca de identidad.
    Offcanvas(bs::navbar::Panel),

    /// Contenido en un [`Panel`](crate::theme::bs::navbar::Panel) lateral, con marca de identidad
    /// a la izquierda y botón de despliegue a la derecha.
    OffcanvasBrandLeft(Embed<Brand>, bs::navbar::Panel),

    /// Contenido en un [`Panel`](crate::theme::bs::navbar::Panel) lateral, con botón de despliegue
    /// a la izquierda y marca de identidad a la derecha.
    OffcanvasBrandRight(Embed<Brand>, bs::navbar::Panel),
}
