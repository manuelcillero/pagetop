use crate::core::component::Context;
use crate::core::theme::{Breakpoint, Responsive};
use crate::html::PropsOp;
use crate::html::unit::UnitValue;
use crate::{AutoDefault, Getters, builder_impl, util};

/// Configuración de márgenes externos por lado lógico y punto de corte.
///
/// No tiene relación con Flexbox. Se aplica sobre cualquier componente, pasándolo directamente al
/// `with_prop()` que suele exponer cualquier componente, gracias a su `From` hacia [`PropsOp`].
///
/// Cada lado admite cualquier [`UnitValue`], incluido [`UnitValue::Auto`] (por ejemplo, para
/// centrar un bloque con `margin-inline: auto`). Los lados lógicos `start`/`end` se traducen a
/// `margin-inline-start`/`margin-inline-end`, respetando LTR/RTL.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// // Centra el bloque horizontalmente y añade espacio inferior.
/// let panel = Container::new().with_prop(
///     Margin::new()
///         .with_x(UnitValue::Auto)
///         .with_bottom(UnitValue::RelRem(1.5)),
/// );
/// ```
///
/// [`Flex`]: crate::base::component::Flex
/// [`FlexItem`]: crate::html::flex::FlexItem
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq, Getters)]
pub struct Margin {
    /// Devuelve el margen superior, por punto de corte.
    #[getters(copy)]
    top: Responsive<UnitValue>,
    /// Devuelve el margen inferior, por punto de corte.
    #[getters(copy)]
    bottom: Responsive<UnitValue>,
    /// Devuelve el margen del lado lógico de inicio, por punto de corte.
    #[getters(copy)]
    start: Responsive<UnitValue>,
    /// Devuelve el margen del lado lógico de fin, por punto de corte.
    #[getters(copy)]
    end: Responsive<UnitValue>,
}

#[builder_impl]
impl Margin {
    /// Crea una configuración de margen sin ningún lado establecido.
    pub fn new() -> Self {
        Self::default()
    }

    // **< Margin BUILDER >*************************************************************************

    /// Establece el margen superior.
    pub fn with_top(mut self, value: UnitValue) -> Self {
        self.top = self.top.set(value);
        self
    }

    /// Establece el margen superior, a partir del punto de corte indicado.
    pub fn with_top_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.top = self.top.set_at(bp, value);
        self
    }

    /// Establece el margen inferior.
    pub fn with_bottom(mut self, value: UnitValue) -> Self {
        self.bottom = self.bottom.set(value);
        self
    }

    /// Establece el margen inferior, a partir del punto de corte indicado.
    pub fn with_bottom_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.bottom = self.bottom.set_at(bp, value);
        self
    }

    /// Establece el margen del lado lógico de inicio (`margin-inline-start`).
    pub fn with_start(mut self, value: UnitValue) -> Self {
        self.start = self.start.set(value);
        self
    }

    /// Establece el margen del lado lógico de inicio (`margin-inline-start`), a partir del punto de
    /// corte indicado.
    pub fn with_start_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.start = self.start.set_at(bp, value);
        self
    }

    /// Establece el margen del lado lógico de fin (`margin-inline-end`).
    pub fn with_end(mut self, value: UnitValue) -> Self {
        self.end = self.end.set(value);
        self
    }

    /// Establece el margen del lado lógico de fin (`margin-inline-end`), a partir del punto de
    /// corte indicado.
    pub fn with_end_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.end = self.end.set_at(bp, value);
        self
    }

    /// Establece el mismo margen en ambos lados lógicos laterales (inicio y fin).
    pub fn with_x(mut self, value: UnitValue) -> Self {
        self.start = self.start.set(value);
        self.end = self.end.set(value);
        self
    }

    /// Establece el mismo margen en ambos lados lógicos laterales (inicio y fin), a partir del
    /// punto de corte indicado.
    pub fn with_x_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.start = self.start.set_at(bp, value);
        self.end = self.end.set_at(bp, value);
        self
    }

    /// Establece el mismo margen arriba y abajo.
    pub fn with_y(mut self, value: UnitValue) -> Self {
        self.top = self.top.set(value);
        self.bottom = self.bottom.set(value);
        self
    }

    /// Establece el mismo margen arriba y abajo, a partir del punto de corte indicado.
    pub fn with_y_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.top = self.top.set_at(bp, value);
        self.bottom = self.bottom.set_at(bp, value);
        self
    }

    /// Establece el mismo margen en los cuatro lados.
    pub fn with_all(mut self, value: UnitValue) -> Self {
        self.top = self.top.set(value);
        self.bottom = self.bottom.set(value);
        self.start = self.start.set(value);
        self.end = self.end.set(value);
        self
    }

    /// Establece el mismo margen en los cuatro lados, a partir del punto de corte indicado.
    pub fn with_all_at(mut self, bp: Breakpoint, value: UnitValue) -> Self {
        self.top = self.top.set_at(bp, value);
        self.bottom = self.bottom.set_at(bp, value);
        self.start = self.start.set_at(bp, value);
        self.end = self.end.set_at(bp, value);
        self
    }
}

impl Margin {
    /// Combina esta configuración con otra `Margin`, lado a lado y punto de corte a punto de corte.
    /// Donde `margin` tenga un valor establecido, sustituye al de `self`, y donde no lo tenga, se
    /// conserva el de `self`. Es el método que usa [`Props::with_prop()`] para que sucesivas
    /// [`PropsOp::Margin`] sobre el mismo componente vayan completando lados concretos sin repetir
    /// los ya establecidos.
    ///
    /// [`Props::with_prop()`]: crate::html::Props::with_prop
    /// [`PropsOp::Margin`]: crate::html::PropsOp::Margin
    pub fn merge(mut self, margin: Margin) -> Self {
        self.top = self.top.merge(margin.top);
        self.bottom = self.bottom.merge(margin.bottom);
        self.start = self.start.merge(margin.start);
        self.end = self.end.merge(margin.end);
        self
    }

    /// Aplica esta configuración como clases de utilidad responsive en el [`Context`], igual que
    /// [`FlexItem::apply()`](crate::html::flex::FlexItem::apply). Un lado sin ningún valor
    /// establecido, o con [`UnitValue::None`], no añade nada.
    ///
    /// Las clases generadas se añaden a `classes`, separadas con un espacio de las que ya hubiera.
    #[rustfmt::skip]
    pub(crate) fn apply(self, cx: &mut Context, classes: &mut String) {
        use crate::html::responsive::{apply, responsive_class, styles, value_to_token};

        apply!(cx, classes, self.top,    "_margin-top_",    "margin-top", val);
        apply!(cx, classes, self.bottom, "_margin-bottom_", "margin-bottom", val);
        apply!(cx, classes, self.start,  "_margin-start_",  "margin-inline-start", val);
        apply!(cx, classes, self.end,    "_margin-end_",    "margin-inline-end", val);
    }
}

impl From<Margin> for PropsOp {
    fn from(margin: Margin) -> Self {
        Self::margin(margin)
    }
}
