use crate::core::component::Context;
use crate::core::theme::{Breakpoint, Responsive};
use crate::html::PropsOp;
use crate::html::unit::UnitValue;
use crate::{AutoDefault, Getters, builder_impl, util};

/// Configuración de relleno interno por lado lógico y punto de corte.
///
/// Mismo mecanismo y criterio de uso que [`Margin`](super::Margin): no tiene relación con Flexbox,
/// y se aplica sobre cualquier componente pasándolo directamente a su `with_prop()`, gracias a su
/// `From` hacia [`PropsOp`].
///
/// A diferencia de `Margin`, [`UnitValue::Auto`] no tiene efecto en ningún lado ya que CSS no
/// admite `padding: auto`, así que un lado establecido a `Auto` se ignora como si no se hubiera
/// establecido.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let card = Container::new().with_prop(
///     Padding::new()
///         .with_all(UnitValue::RelRem(1.0))
///         .with_bottom_at(Breakpoint::Md, UnitValue::RelRem(2.0)),
/// );
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq, Getters)]
pub struct Padding {
    /// Devuelve el relleno interno superior, por punto de corte.
    #[getters(copy)]
    top: Responsive<UnitValue>,
    /// Devuelve el relleno interno inferior, por punto de corte.
    #[getters(copy)]
    bottom: Responsive<UnitValue>,
    /// Devuelve el relleno interno del lado lógico de inicio, por punto de corte.
    #[getters(copy)]
    start: Responsive<UnitValue>,
    /// Devuelve el relleno interno del lado lógico de fin, por punto de corte.
    #[getters(copy)]
    end: Responsive<UnitValue>,
}

#[builder_impl]
impl Padding {
    /// Crea una configuración de relleno sin ningún lado establecido.
    pub fn new() -> Self {
        Self::default()
    }

    // **< Padding BUILDER >************************************************************************

    /// Establece el relleno interno superior.
    pub fn with_top(mut self, value: UnitValue) -> Self {
        self.top = self.top.set(value);
        self
    }

    /// Establece el relleno interno superior, a partir del punto de corte indicado.
    pub fn with_top_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.top = self.top.set_at(bp, value);
        self
    }

    /// Establece el relleno interno inferior.
    pub fn with_bottom(mut self, value: UnitValue) -> Self {
        self.bottom = self.bottom.set(value);
        self
    }

    /// Establece el relleno interno inferior, a partir del punto de corte indicado.
    pub fn with_bottom_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.bottom = self.bottom.set_at(bp, value);
        self
    }

    /// Establece el relleno interno del lado lógico de inicio (`padding-inline-start`).
    pub fn with_start(mut self, value: UnitValue) -> Self {
        self.start = self.start.set(value);
        self
    }

    /// Establece el relleno interno del lado lógico de inicio (`padding-inline-start`), a partir
    /// del punto de corte indicado.
    pub fn with_start_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.start = self.start.set_at(bp, value);
        self
    }

    /// Establece el relleno interno del lado lógico de fin (`padding-inline-end`).
    pub fn with_end(mut self, value: UnitValue) -> Self {
        self.end = self.end.set(value);
        self
    }

    /// Establece el relleno interno del lado lógico de fin (`padding-inline-end`), a partir del
    /// punto de corte indicado.
    pub fn with_end_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.end = self.end.set_at(bp, value);
        self
    }

    /// Establece el mismo relleno interno en ambos lados lógicos laterales (inicio y fin).
    pub fn with_x(mut self, value: UnitValue) -> Self {
        self.start = self.start.set(value);
        self.end = self.end.set(value);
        self
    }

    /// Establece el mismo relleno interno en ambos lados lógicos laterales (inicio y fin), a partir
    /// del punto de corte indicado.
    pub fn with_x_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.start = self.start.set_at(bp, value);
        self.end = self.end.set_at(bp, value);
        self
    }

    /// Establece el mismo relleno interno arriba y abajo.
    pub fn with_y(mut self, value: UnitValue) -> Self {
        self.top = self.top.set(value);
        self.bottom = self.bottom.set(value);
        self
    }

    /// Establece el mismo relleno interno arriba y abajo, a partir del punto de corte indicado.
    pub fn with_y_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.top = self.top.set_at(bp, value);
        self.bottom = self.bottom.set_at(bp, value);
        self
    }

    /// Establece el mismo relleno interno en los cuatro lados.
    pub fn with_all(mut self, value: UnitValue) -> Self {
        self.top = self.top.set(value);
        self.bottom = self.bottom.set(value);
        self.start = self.start.set(value);
        self.end = self.end.set(value);
        self
    }

    /// Establece el mismo relleno interno en los cuatro lados, a partir del punto de corte
    /// indicado.
    pub fn with_all_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.top = self.top.set_at(bp, value);
        self.bottom = self.bottom.set_at(bp, value);
        self.start = self.start.set_at(bp, value);
        self.end = self.end.set_at(bp, value);
        self
    }
}

impl Padding {
    /// Combina esta configuración con otra `Padding`, lado a lado y punto de corte a punto de
    /// corte; mismo criterio que [`Margin::merge()`](super::Margin::merge).
    pub fn merge(mut self, padding: Padding) -> Self {
        self.top = self.top.merge(padding.top);
        self.bottom = self.bottom.merge(padding.bottom);
        self.start = self.start.merge(padding.start);
        self.end = self.end.merge(padding.end);
        self
    }

    /// Aplica esta configuración como clases de utilidad responsive en el [`Context`], igual que
    /// [`Margin::apply()`](super::Margin::apply), salvo que aquí un lado con [`UnitValue::Auto`] se
    /// descarta (ver la documentación de este tipo).
    ///
    /// Las clases generadas se añaden a `classes`, separadas con un espacio de las que ya hubiera.
    #[rustfmt::skip]
    pub(crate) fn apply(self, cx: &mut Context, classes: &mut String) {
        Self::apply_side(cx, classes, self.top,    "_padding-top_",    "padding-top");
        Self::apply_side(cx, classes, self.bottom, "_padding-bottom_", "padding-bottom");
        Self::apply_side(cx, classes, self.start,  "_padding-start_",  "padding-inline-start");
        Self::apply_side(cx, classes, self.end,    "_padding-end_",    "padding-inline-end");
    }

    // Aplica un único lado, descartando `UnitValue::Auto` porque CSS no admite `padding: auto`.
    fn apply_side(
        cx: &mut Context,
        classes: &mut String,
        field: Responsive<UnitValue>,
        prefix: &'static str,
        property: &'static str,
    ) {
        use crate::html::responsive::{responsive_class, styles, value_to_token};

        for (bp, value) in field.by_breakpoint() {
            if value != UnitValue::Auto {
                let value = value.value();
                if !value.is_empty() {
                    let entry = bp.resolved(cx);
                    let class = responsive_class!(prefix, value_to_token(&value), entry);
                    styles(cx, classes, entry, class.into(), property, value);
                }
            }
        }
    }
}

impl From<Padding> for PropsOp {
    fn from(padding: Padding) -> Self {
        Self::padding(padding)
    }
}
