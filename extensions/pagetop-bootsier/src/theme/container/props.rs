use pagetop::prelude::*;

use crate::theme::aux::BreakPoint;

// **< Kind >***************************************************************************************

/// Tipo de contenedor ([`Container`](crate::theme::Container)).
///
/// Permite aplicar la etiqueta HTML apropiada (`<main>`, `<header>`, etc.) manteniendo una API
/// común a todos los contenedores.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// Contenedor genérico (`<div>`).
    #[default]
    Default,
    /// Contenido principal de la página (`<main>`).
    Main,
    /// Encabezado de la página o de sección (`<header>`).
    Header,
    /// Pie de la página o de sección (`<footer>`).
    Footer,
    /// Sección de contenido (`<section>`).
    Section,
    /// Artículo de contenido (`<article>`).
    Article,
}

// **< Width >**************************************************************************************

/// Define cómo se comporta el ancho de un contenedor ([`Container`](crate::theme::Container)).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Width {
    /// Comportamiento por defecto, aplica los anchos máximos predefinidos para cada punto de
    /// ruptura. Por debajo del menor punto de ruptura ocupa el 100% del ancho disponible.
    #[default]
    Default,
    /// Aplica los anchos máximos predefinidos a partir del punto de ruptura indicado. Por debajo de
    /// ese punto de ruptura ocupa el 100% del ancho disponible.
    From(BreakPoint),
    /// Ocupa el 100% del ancho disponible siempre.
    Fluid,
    /// Ocupa el 100% del ancho disponible hasta un ancho máximo explícito.
    FluidMax(UnitValue),
}

impl Width {
    const CONTAINER: &str = "container";

    /* Añade el comportamiento del contenedor a la cadena de clases según ancho (reservado).
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        match self {
            Self::Default => BreakPoint::None.push_class(classes, Self::CONTAINER, ""),
            Self::From(bp) => bp.push_class(classes, Self::CONTAINER, ""),
            Self::Fluid | Self::FluidMax(_) => {
                BreakPoint::None.push_class(classes, Self::CONTAINER, "fluid")
            }
        }
    } */

    /// Devuelve la clase asociada al comportamiento del contenedor según el ajuste de su ancho.
    pub fn to_class(self) -> String {
        match self {
            Self::Default => BreakPoint::None.class_with(Self::CONTAINER, ""),
            Self::From(bp) => bp.class_with(Self::CONTAINER, ""),
            Self::Fluid | Self::FluidMax(_) => {
                BreakPoint::None.class_with(Self::CONTAINER, "fluid")
            }
        }
    }
}
