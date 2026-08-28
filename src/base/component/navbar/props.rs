use crate::prelude::*;

// **< Layout >*************************************************************************************

/// Representa las distintas formas de presentación de una barra de navegación
/// [`Navbar`](super::Navbar).
///
/// Sólo recoge las combinaciones de marca y botón de despliegue independientes de cualquier
/// framework CSS. Un tema puede definir su propia variante de disposición (posiciones fijas,
/// contenido en un panel lateral...) con su propio tipo, sin depender de éste.
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
