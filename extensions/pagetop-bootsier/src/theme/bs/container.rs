//! Definiciones para crear contenedores de componentes ([`Container`]).
//!
//! El comportamiento del ancho del contenedor se establece con [`Container::with_width()`], usando
//! [`Width`]. Sin él, Bootsier aplica los anchos máximos predefinidos de Bootstrap para cada punto
//! de corte ([`Width::Responsive`]).
//!
//! También se pueden aplicar al componente clases predefinidas para:
//!
//! - Modificar el color de fondo ([`Bg`](crate::theme::class::Bg)).
//! - Definir la apariencia del texto ([`Text`](crate::theme::class::Text)).
//! - Establecer bordes ([`Border`](crate::theme::class::Border)).
//! - Redondear las esquinas ([`Rounded`](crate::theme::class::Rounded)).
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//! use pagetop_bootsier::theme::*;
//!
//! let main = bs::Container::main()
//!     .with_id("main-page")
//!     .with_width(bs::container::Width::From(Breakpoint::Lg))
//!     .with_prop(PropsOp::add_classes(class::Bg::with(BootsierColors::Light)))
//!     .with_prop(PropsOp::add_classes(class::Text::with(BootsierColors::Dark)))
//!     .with_prop(PropsOp::add_classes(class::Border::with(ScaleSize::One)))
//!     .with_prop(PropsOp::add_classes(class::Rounded::new()));
//! ```

use pagetop::prelude::*;

pub use pagetop::base::component::container::{Container, Kind, Width};

// **< Container SETUP >****************************************************************************

pub(crate) fn setup(container: &mut Container) {
    if container.width().is_none() {
        container.alter_prop(PropsOp::prepend_classes("container"));
    }
}
