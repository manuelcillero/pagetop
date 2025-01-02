use pagetop::prelude::*;

use std::fmt;

mod container;
pub use container::{Container, ContainerType};

pub mod grid;

/// Define los puntos de interrupción (*breakpoints*) usados por Bootstrap para diseño responsivo.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum BreakPoint {
    #[default]
    None,  //  <  576px (Dispositivos muy pequeños: teléfonos en modo vertical, menos de 576px)
    SM,    // >=  576px (Dispositivos pequeños: teléfonos en modo horizontal, 576px o más)
    MD,    // >=  768px (Dispositivos medianos: tabletas, 768px o más)
    LG,    // >=  992px (Dispositivos grandes: puestos de escritorio, 992px o más)
    XL,    // >= 1200px (Dispositivos muy grandes: puestos de escritorio grandes, 1200px o más)
    XXL,   // >= 1400px (Dispositivos extragrandes: puestos de escritorio más grandes, 1400px o más)
    Fluid, // Siempre aplica el 100% del dispositivo
}

impl BreakPoint {
    /// Indica si se trata de un punto de interrupción de Bootstrap.
    /// Devuelve `true` si el valor es SM, MD, LG, XL o XXL.
    /// Devuelve `false` si es None o Fluid.
    pub fn is_breakpoint(&self) -> bool {
        !matches!(self, BreakPoint::None | BreakPoint::Fluid)
    }
}

/// Devuelve el texto asociado al punto de interrupción usado por Bootstrap.
#[rustfmt::skip]
impl fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BreakPoint::None  => write!(f, ""),
            BreakPoint::SM    => write!(f, "sm"),
            BreakPoint::MD    => write!(f, "md"),
            BreakPoint::LG    => write!(f, "lg"),
            BreakPoint::XL    => write!(f, "xl"),
            BreakPoint::XXL   => write!(f, "xxl"),
            BreakPoint::Fluid => write!(f, "fluid"),
        }
    }
}
