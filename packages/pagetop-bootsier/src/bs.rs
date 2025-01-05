use pagetop::prelude::*;

use std::fmt;

// Container.
pub mod container;
pub use container::{Container, ContainerType};

// Grid.
pub mod grid;
pub use grid::Grid;

// Offcanvas.
pub mod offcanvas;
pub use offcanvas::{
    Offcanvas, OffcanvasBackdrop, OffcanvasBodyScroll, OffcanvasPlacement, OffcanvasVisibility,
};

// Navbar.
pub mod navbar;
pub use navbar::{Navbar, NavbarType};

/// Define los puntos de interrupción (*breakpoints*) usados por Bootstrap para diseño responsivo.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum BreakPoint {
    #[default]  // DIMENSIONES - DISPOSITIVOS ---------------------------------------------------
    None,       //   <  576px    Muy pequeños: teléfonos en modo vertical, menos de 576px
    SM,         //  >=  576px    Pequeños: teléfonos en modo horizontal, 576px o más
    MD,         //  >=  768px    Medianos: tabletas, 768px o más
    LG,         //  >=  992px    Grandes: puestos de escritorio, 992px o más
    XL,         //  >= 1200px    Muy grandes: puestos de escritorio grandes, 1200px o más
    XXL,        //  >= 1400px    Extragrandes: puestos de escritorio más grandes, 1400px o más
                // ------------------------------------------------------------------------------
    Fluid,                // Para Container, aplica el 100% del dispositivo siempre
    FluidMax(unit::Value) // Para Container, aplica el 100% del dispositivo hasta un ancho máximo
}

impl BreakPoint {
    /// Indica si el punto de interrupción es efectivo en Bootstrap.
    ///
    /// Devuelve `true` si el valor es `SM`, `MD`, `LG`, `XL` o `XXL`.
    /// Devuelve `false` si es `None`, `Fluid` o `FluidMax`.
    pub fn is_breakpoint(&self) -> bool {
        !matches!(
            self,
            BreakPoint::None | BreakPoint::Fluid | BreakPoint::FluidMax(_)
        )
    }

    /// Genera un nombre de clase CSS basado en el punto de interrupción.
    ///
    /// Si es un punto de interrupción efectivo (ver [`is_breakpoint()`] se concatenan el prefijo
    /// proporcionado, un guion (`-`) y el texto asociado al punto de interrupción. En otro caso, se
    /// devuelve únicamente el prefijo.
    ///
    /// # Parámetros
    ///
    /// - `prefix`: Prefijo para concatenar con el punto de interrupción.
    ///
    /// # Ejemplo
    ///
    /// ```rust#ignore
    /// let breakpoint = BreakPoint::MD;
    /// let class = breakpoint.breakpoint_class("col");
    /// assert_eq!(class, "col-md".to_string());
    ///
    /// let breakpoint = BreakPoint::Fluid;
    /// let class = breakpoint.breakpoint_class("offcanvas");
    /// assert_eq!(class, "offcanvas".to_string());
    /// ```
    pub fn breakpoint_class(&self, prefix: impl Into<String>) -> String {
        let prefix: String = prefix.into();
        if self.is_breakpoint() {
            join_string!(prefix, "-", self.to_string())
        } else {
            prefix
        }
    }
}

/// Devuelve el texto asociado al punto de interrupción usado por Bootstrap.
#[rustfmt::skip]
impl fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BreakPoint::None        => write!(f, ""),
            BreakPoint::SM          => write!(f, "sm"),
            BreakPoint::MD          => write!(f, "md"),
            BreakPoint::LG          => write!(f, "lg"),
            BreakPoint::XL          => write!(f, "xl"),
            BreakPoint::XXL         => write!(f, "xxl"),
            BreakPoint::Fluid       => write!(f, "fluid"),
            BreakPoint::FluidMax(_) => write!(f, "fluid"),
        }
    }
}
