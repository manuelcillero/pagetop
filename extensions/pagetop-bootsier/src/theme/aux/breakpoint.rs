use pagetop::prelude::*;

use std::fmt;

/// Define los puntos de ruptura (*breakpoints*) para aplicar diseño *responsive*.
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
}

impl BreakPoint {
    #[rustfmt::skip]
    #[inline]
    const fn suffix(&self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::SM   => Some("sm"),
            Self::MD   => Some("md"),
            Self::LG   => Some("lg"),
            Self::XL   => Some("xl"),
            Self::XXL  => Some("xxl"),
        }
    }

    /// Genera un nombre de clase CSS basado en el punto de ruptura.
    ///
    /// Si es un punto de ruptura efectivo concatena el prefijo, un guion (`-`) y el sufijo
    /// asociado. Para `None` devuelve sólo el prefijo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// let breakpoint = BreakPoint::MD;
    /// assert_eq!(breakpoint.to_class("col"), "col-md");
    ///
    /// let breakpoint = BreakPoint::None;
    /// assert_eq!(breakpoint.to_class("offcanvas"), "offcanvas");
    /// ```
    #[inline]
    pub fn to_class(&self, prefix: impl AsRef<str>) -> String {
        join_pair!(prefix, "-", self.suffix().unwrap_or_default())
    }

    /// Intenta generar un nombre de clase CSS basado en el punto de ruptura.
    ///
    /// Si es un punto de ruptura efectivo devuelve `Some(String)` concatenando el prefijo, un guion
    /// (`-`) y el sufijo asociado. En otro caso devuelve `None`.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop_bootsier::prelude::*;
    /// let breakpoint = BreakPoint::MD;
    /// let class = breakpoint.try_class("col");
    /// assert_eq!(class, Some("col-md".to_string()));
    ///
    /// let breakpoint = BreakPoint::None;
    /// let class = breakpoint.try_class("navbar-expand");
    /// assert_eq!(class, None);
    /// ```
    #[inline]
    pub fn try_class(&self, prefix: impl AsRef<str>) -> Option<String> {
        self.suffix().map(|suffix| join_pair!(prefix, "-", suffix))
    }
}

impl fmt::Display for BreakPoint {
    /// Implementa [`Display`](std::fmt::Display) para asociar `"sm"`, `"md"`, `"lg"`, `"xl"` o
    /// `"xxl"` a los puntos de ruptura `BreakPoint::SM`, `MD`, `LG`, `XL` o `XXL`, respectivamente.
    /// Y `""` (cadena vacía) a `BreakPoint::None`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(suffix) = self.suffix() {
            f.write_str(suffix)
        } else {
            Ok(())
        }
    }
}
