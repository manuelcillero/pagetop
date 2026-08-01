//! Definiciones para crear contenedores de componentes ([`Container`]).

use pagetop::prelude::*;

use crate::theme::*;

pub use pagetop::base::component::container::{Container, Kind};

const EXTRA_WIDTH: &str = "bootsier.container.width";

/// Extensión de Bootsier para [`Container`].
///
/// Permite establecer el comportamiento del ancho del contenedor usando el método
/// [`with_width()`](Self::with_width).
///
/// También habilita al componente para aceptar clases predefinidas para:
///
/// - Modificar el color de fondo ([`Bg`](crate::theme::class::Bg)).
/// - Definir la apariencia del texto ([`Text`](crate::theme::class::Text)).
/// - Establecer bordes ([`Border`](crate::theme::class::Border)).
/// - Redondear las esquinas ([`Rounded`](crate::theme::class::Rounded)).
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let main = bs::Container::main()
///     .with_id("main-page")
///     .with_width(bs::container::Width::From(BreakPoint::LG))
///     .with_prop(PropsOp::add_classes(class::Bg::with(ThemeColor::Light)))
///     .with_prop(PropsOp::add_classes(class::Text::with(ThemeColor::Dark)))
///     .with_prop(PropsOp::add_classes(class::Border::with(ScaleSize::One)))
///     .with_prop(PropsOp::add_classes(class::Rounded::new()));
/// ```
pub trait ContainerBootsier {
    /// Establece el comportamiento del ancho para el contenedor.
    ///
    /// Determina si el contenedor aplica los anchos máximos predefinidos para cada punto de
    /// ruptura, o si ocupa siempre el 100% del ancho disponible, o lo hace hasta un ancho máximo
    /// explícito. Ver [`Width`] para las variantes disponibles.
    #[builder_fn]
    fn with_width(self, width: Width) -> Self;
}

impl ContainerBootsier for Container {
    #[builder_fn]
    fn with_width(mut self, width: Width) -> Self {
        self.alter_prop(PropsOp::set_extra(EXTRA_WIDTH, width));
        self
    }
}

// **< Width >**************************************************************************************

/// Define cómo se comporta el ancho de un contenedor ([`Container`]).
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

    /// Añade la clase asociada al ancho del contenedor a la cadena de clases.
    #[inline]
    pub fn push_to(self, classes: &mut String) {
        match self {
            Self::Default => BreakPoint::None.push_to(classes, Self::CONTAINER, ""),
            Self::From(bp) => bp.push_to(classes, Self::CONTAINER, ""),
            Self::Fluid | Self::FluidMax(_) => {
                BreakPoint::None.push_to(classes, Self::CONTAINER, "fluid")
            }
        }
    }

    /// Devuelve la clase asociada al ancho del contenedor.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

// **< Container SETUP >****************************************************************************

pub(crate) fn setup(container: &mut Container) {
    let width = container.props().extra_or(EXTRA_WIDTH, Width::default());
    container.alter_prop(PropsOp::prepend_classes(width.to_class()));
    if let Width::FluidMax(w) = width
        && w.is_measurable()
    {
        container.alter_prop(PropsOp::add_style("max-width", w.to_string()));
    }
}
