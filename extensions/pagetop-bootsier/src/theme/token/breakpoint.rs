use pagetop::prelude::*;

/// Puntos de ruptura (*breakpoints*) para aplicar diseño *responsive*.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum BreakPoint {
    /// **Menos de 576px**. Dispositivos muy pequeños: teléfonos en modo vertical.
    #[default]
    None,
    /// **576px o más**. Dispositivos pequeños: teléfonos en modo horizontal.
    SM,
    /// **768px o más**. Dispositivos medianos: tabletas.
    MD,
    /// **992px o más**. Dispositivos grandes: puestos de escritorio.
    LG,
    /// **1200px o más**. Dispositivos muy grandes: puestos de escritorio grandes.
    XL,
    /// **1400px o más**. Dispositivos extragrandes: puestos de escritorio más grandes.
    XXL,
}

impl BreakPoint {
    /// Devuelve la identificación del punto de ruptura (`"sm"`, `"md"`, etc.), o `""` para `None`.
    #[rustfmt::skip]
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "",
            Self::SM   => "sm",
            Self::MD   => "md",
            Self::LG   => "lg",
            Self::XL   => "xl",
            Self::XXL  => "xxl",
        }
    }

    // Añade el punto de ruptura con un prefijo y un sufijo (opcional) a la cadena de clases.
    //
    // - Para `None`: `prefix` o `prefix-suffix` (si `suffix` no está vacío).
    // - Para `SM..XXL`: `prefix-{breakpoint}` o `prefix-{breakpoint}-{suffix}`.
    #[inline]
    pub(crate) fn push_to(self, classes: &mut String, prefix: &str, suffix: &str) {
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
}
