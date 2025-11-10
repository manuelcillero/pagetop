use pagetop::prelude::*;

use crate::prelude::*;

// **< Kind >***************************************************************************************

/// Tipo de contenedor ([`Container`]).
///
/// Permite aplicar la etiqueta HTML apropiada (`<main>`, `<header>`, etc.) manteniendo una API
/// común a todos los contenedores.
#[derive(AutoDefault)]
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

/// Define el comportamiento para ajustar el ancho de un contenedor ([`Container`]).
#[derive(AutoDefault)]
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
