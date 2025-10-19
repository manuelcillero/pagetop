use pagetop::prelude::*;

use std::fmt;

/// Define los puntos de ruptura (*breakpoints*) para aplicar diseño *responsive*.
///
/// - `"sm"`, `"md"`, `"lg"`, `"xl"` o `"xxl"` para los puntos de ruptura `SM`, `MD`, `LG`, `XL` o
///   `XXL`, respectivamente.
/// - `""` (cadena vacía) para `None`.
/// - `"fluid"` para las variantes `Fluid` y `FluidMax(_)`, útil para modelar `container-fluid` en
///   [`Container`](crate::theme::Container). Se debe tener en cuenta que `"fluid"` **no** es un
///   sufijo de *breakpoint*. Para construir clases válidas con prefijo (p. ej., `"col-md"`), se
///   recomienda usar `to_class()` (Self::to_class) o `try_class()` (Self::try_class).
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// assert_eq!(BreakPoint::MD.to_string(), "md");
/// assert_eq!(BreakPoint::None.to_string(), "");
/// assert_eq!(BreakPoint::Fluid.to_string(), "fluid");
///
/// // Forma correcta para clases con prefijo:
/// //assert_eq!(BreakPoint::MD.to_class("col"), "col-md");
/// //assert_eq!(BreakPoint::Fluid.to_class("offcanvas"), "offcanvas");
/// ```
#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum BreakPoint {
    /// **Menos de 576px**. Dispositivos muy pequeños: teléfonos en modo vertical.
    #[default]
    None,
    /// **576px o más** - Dispositivos pequeños: teléfonos en modo horizontal.
    SM,
    /// **768px o más** - Dispositivos medianos: tabletas.
    MD,
    /// **992px o más** - Dispositivos grandes: puestos de escritorio.
    LG,
    /// **1200px o más** - Dispositivos muy grandes: puestos de escritorio grandes.
    XL,
    /// **1400px o más** - Dispositivos extragrandes: puestos de escritorio más grandes.
    XXL,
    /// Para [`Container`](crate::theme::Container), ocupa el 100% del ancho del dispositivo.
    Fluid,
    /// Para [`Container`](crate::theme::Container), ocupa el 100% del ancho del dispositivo hasta
    /// un ancho máximo indicado.
    FluidMax(UnitValue)
}

impl BreakPoint {
    /// Comprueba si es un punto de ruptura efectivo.
    ///
    /// Devuelve `true` si el valor es `SM`, `MD`, `LG`, `XL` o `XXL`; y `false` en otro caso.
    #[inline]
    pub const fn is_breakpoint(&self) -> bool {
        matches!(self, Self::SM | Self::MD | Self::LG | Self::XL | Self::XXL)
    }

    /// Genera un nombre de clase CSS basado en el punto de ruptura.
    ///
    /// Si es un punto de ruptura efectivo (ver [`is_breakpoint()`](Self::is_breakpoint), concatena
    /// el prefijo, un guion (`-`) y el sufijo asociado. En otro caso devuelve únicamente el
    /// prefijo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// let breakpoint = BreakPoint::MD;
    /// let class = breakpoint.to_class("col");
    /// assert_eq!(class, "col-md".to_string());
    ///
    /// let breakpoint = BreakPoint::Fluid;
    /// let class = breakpoint.to_class("offcanvas");
    /// assert_eq!(class, "offcanvas".to_string());
    /// ```
    #[inline]
    pub fn to_class(&self, prefix: impl AsRef<str>) -> String {
        if self.is_breakpoint() {
            join!(prefix, "-", self.to_string())
        } else {
            String::from(prefix.as_ref())
        }
    }

    /// Intenta generar un nombre de clase CSS basado en el punto de ruptura.
    ///
    /// Si es un punto de ruptura efectivo (ver [`is_breakpoint()`](Self::is_breakpoint), devuelve
    /// `Some(String)` concatenando el prefijo, un guion (`-`) y el sufijo asociado. En otro caso
    /// devuelve `None`.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// let breakpoint = BreakPoint::MD;
    /// let class = breakpoint.try_class("col");
    /// assert_eq!(class, Some("col-md".to_string()));
    ///
    /// let breakpoint = BreakPoint::Fluid;
    /// let class = breakpoint.try_class("navbar-expanded");
    /// assert_eq!(class, None);
    /// ```
    #[inline]
    pub fn try_class(&self, prefix: impl AsRef<str>) -> Option<String> {
        if self.is_breakpoint() {
            Some(join!(prefix, "-", self.to_string()))
        } else {
            None
        }
    }
}

#[rustfmt::skip]
impl fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None        => Ok(()),
            Self::SM          => f.write_str("sm"),
            Self::MD          => f.write_str("md"),
            Self::LG          => f.write_str("lg"),
            Self::XL          => f.write_str("xl"),
            Self::XXL         => f.write_str("xxl"),
            // Devuelven "fluid" (para modelar `container-fluid`).
            Self::Fluid       => f.write_str("fluid"),
            Self::FluidMax(_) => f.write_str("fluid"),
        }
    }
}
