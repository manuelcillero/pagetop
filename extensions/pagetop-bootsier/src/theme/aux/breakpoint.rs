use pagetop::prelude::*;

/// Define los puntos de ruptura (*breakpoints*) para aplicar diseño *responsive*.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
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
    // Devuelve la identificación del punto de ruptura.
    #[rustfmt::skip]
    #[inline]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::None => "",
            Self::SM   => "sm",
            Self::MD   => "md",
            Self::LG   => "lg",
            Self::XL   => "xl",
            Self::XXL  => "xxl",
        }
    }

    // Añade el punto de ruptura con un prefijo y un sufijo (opcional) separados por un guion `-` a
    // la cadena de clases.
    //
    // - Para `None` - `prefix` o `prefix-suffix` (si `suffix` no está vacío).
    // - Para `SM..XXL` - `prefix-{breakpoint}` o `prefix-{breakpoint}-{suffix}`.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String, prefix: &str, suffix: &str) {
        if prefix.is_empty() {
            return;
        }
        if !classes.is_empty() {
            classes.push(' ');
        }
        match self {
            Self::None => classes.push_str(prefix),
            _ => {
                classes.push_str(prefix);
                classes.push('-');
                classes.push_str(self.as_str());
            }
        }
        if !suffix.is_empty() {
            classes.push('-');
            classes.push_str(suffix);
        }
    }

    // Devuelve la clase para el punto de ruptura, con un prefijo y un sufijo opcional, separados
    // por un guion `-`.
    //
    // - Para `None` - `prefix` o `prefix-suffix` (si `suffix` no está vacío).
    // - Para `SM..XXL` - `prefix-{breakpoint}` o `prefix-{breakpoint}-{suffix}`.
    // - Si `prefix` está vacío devuelve `""`.
    //
    // # Ejemplos
    //
    // ```rust
    // # use pagetop_bootsier::prelude::*;
    // let bp = BreakPoint::MD;
    // assert_eq!(bp.class_with("col", ""), "col-md");
    // assert_eq!(bp.class_with("col", "6"), "col-md-6");
    //
    // let bp = BreakPoint::None;
    // assert_eq!(bp.class_with("offcanvas", ""), "offcanvas");
    // assert_eq!(bp.class_with("col", "12"), "col-12");
    //
    // let bp = BreakPoint::LG;
    // assert_eq!(bp.class_with("", "3"), "");
    // ```
    #[inline]
    pub(crate) fn class_with(self, prefix: &str, suffix: &str) -> String {
        if prefix.is_empty() {
            return String::new();
        }

        let bp = self.as_str();
        let has_bp = !bp.is_empty();
        let has_suffix = !suffix.is_empty();

        let mut len = prefix.len();
        if has_bp {
            len += 1 + bp.len();
        }
        if has_suffix {
            len += 1 + suffix.len();
        }
        let mut class = String::with_capacity(len);
        class.push_str(prefix);
        if has_bp {
            class.push('-');
            class.push_str(bp);
        }
        if has_suffix {
            class.push('-');
            class.push_str(suffix);
        }
        class
    }
}
