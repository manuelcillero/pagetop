use crate::core::component::Context;
use crate::core::theme::{Breakpoint, Responsive};
use crate::html::PropsOp;
use crate::html::flex::{ItemAlign, ItemGrow, ItemOffset, ItemOrder, ItemShrink, ItemSize};
use crate::{AutoDefault, Getters, builder_impl, util};

/// Configuración de un elemento como ítem de un contenedor Flexbox.
///
/// A diferencia de [`Flex`](crate::html::flex::Flex), que configura el comportamiento Flexbox
/// global de un contenedor y sus hijos como grupo, `FlexItem` configura un único elemento en
/// relación con el contenedor flex padre: crecimiento ([`ItemGrow`]), reducción ([`ItemShrink`]),
/// alineación individual ([`ItemAlign`]), orden visual ([`ItemOrder`]), tamaño ([`ItemSize`]) y
/// desplazamiento ([`ItemOffset`]).
///
/// No tiene un builder dedicado en ningún componente. De hecho, no tendría sentido porque cualquier
/// componente puede acabar siendo hijo de un contenedor flex, y ninguno debería necesitar un campo
/// propio para esto. Se aplica sobre el `with_prop()` que suele exponer cualquier componente, que
/// acepta `FlexItem` directamente gracias a su `From` hacia [`PropsOp`].
///
/// Con [`ItemSize`] y [`ItemOffset`] se pueden modelar rejillas de columnas fijas sobre Flexbox,
/// combinando un tamaño en fracción del contenedor con un desplazamiento lateral cuando se
/// necesite.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// // Crece para ocupar el espacio sobrante, partiendo de ancho cero.
/// let panel = Button::plain(Lc::n("Panel")).with_prop(
///     FlexItem::new()
///         .with_grow(flex::ItemGrow::Is1)
///         .with_size(flex::ItemSize::Custom(UnitValue::Zero)),
/// );
///
/// // Ocupa un tercio del ancho del contenedor, desplazado otro tercio desde el inicio.
/// let column = Container::new().with_prop(
///     FlexItem::new()
///         .with_size(flex::ItemSize::Percent33)
///         .with_offset(flex::ItemOffset::Percent33),
/// );
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq, Getters)]
pub struct FlexItem {
    /// Devuelve el factor de crecimiento, por punto de corte.
    #[getters(copy)]
    grow: Responsive<ItemGrow>,
    /// Devuelve el factor de reducción, por punto de corte.
    #[getters(copy)]
    shrink: Responsive<ItemShrink>,
    /// Devuelve la alineación individual en el eje transversal, por punto de corte.
    #[getters(copy)]
    align_self: Responsive<ItemAlign>,
    /// Devuelve la posición en el orden visual, por punto de corte.
    #[getters(copy)]
    order: Responsive<ItemOrder>,
    /// Devuelve el tamaño como fracción del contenedor, por punto de corte.
    #[getters(copy)]
    size: Responsive<ItemSize>,
    /// Devuelve el desplazamiento respecto al inicio del contenedor, por punto de corte.
    #[getters(copy)]
    offset: Responsive<ItemOffset>,
}

#[builder_impl]
impl FlexItem {
    /// Crea una configuración de ítem con todos los valores por defecto.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea una configuración de ítem que empuja el elemento, y los que le sigan, hacia el extremo
    /// final de un contenedor flex en fila.
    ///
    /// Aplica `margin-inline-start: auto`, un margen automático que absorbe todo el espacio libre
    /// que quede antes del elemento en el eje de escritura. Con la dirección por defecto
    /// ([`Direction::Row`](super::Direction::Row)) ese eje es el principal, de ahí el efecto de
    /// empuje. En un contenedor en columna, en cambio, ese eje es el transversal: el margen ya no
    /// empuja nada, sólo desplaza ese elemento hacia el final de la línea (a la derecha si se
    /// escribe de izquierda a derecha).
    ///
    /// Es el mecanismo estándar de Flexbox para, por ejemplo, separar dos menús dentro de un mismo
    /// [`Navbar`](crate::base::component::Navbar) (uno pegado al inicio, el siguiente empujado al
    /// final) sin que el contenedor necesite conocer ninguna distinción entre sus elementos.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// use pagetop::prelude::*;
    ///
    /// let user_menu = Nav::new()
    ///     .with_prop(FlexItem::push_end())
    ///     .with_item(nav::Item::link(Lc::n("Profile"), "/profile"))
    ///     .with_item(nav::Item::link(Lc::n("Sign out"), "/sign-out"));
    /// ```
    pub fn push_end() -> Self {
        Self::new().with_offset(ItemOffset::Auto)
    }

    // **< FlexItem BUILDER >***********************************************************************

    /// Establece el factor de crecimiento.
    pub fn with_grow(mut self, grow: ItemGrow) -> Self {
        self.grow = self.grow.set(grow);
        self
    }

    /// Establece el factor de crecimiento, a partir del punto de corte indicado.
    pub fn with_grow_at(mut self, bp: Breakpoint, grow: ItemGrow) -> Self {
        self.grow = self.grow.set_at(bp, grow);
        self
    }

    /// Establece el factor de reducción.
    pub fn with_shrink(mut self, shrink: ItemShrink) -> Self {
        self.shrink = self.shrink.set(shrink);
        self
    }

    /// Establece el factor de reducción, a partir del punto de corte indicado.
    pub fn with_shrink_at(mut self, bp: Breakpoint, shrink: ItemShrink) -> Self {
        self.shrink = self.shrink.set_at(bp, shrink);
        self
    }

    /// Establece la alineación individual en el eje transversal.
    pub fn with_align_self(mut self, align_self: ItemAlign) -> Self {
        self.align_self = self.align_self.set(align_self);
        self
    }

    /// Establece la alineación individual en el eje transversal, a partir del punto de corte
    /// indicado.
    pub fn with_align_self_at(mut self, bp: Breakpoint, align_self: ItemAlign) -> Self {
        self.align_self = self.align_self.set_at(bp, align_self);
        self
    }

    /// Establece la posición en el orden visual.
    pub fn with_order(mut self, order: ItemOrder) -> Self {
        self.order = self.order.set(order);
        self
    }

    /// Establece la posición en el orden visual, a partir del punto de corte indicado.
    pub fn with_order_at(mut self, bp: Breakpoint, order: ItemOrder) -> Self {
        self.order = self.order.set_at(bp, order);
        self
    }

    /// Establece el tamaño como una fracción del contenedor (`flex-basis`). No fuerza
    /// [`ItemShrink::Is0`](super::ItemShrink::Is0) por sí solo (consulta la documentación de
    /// [`ItemSize`] antes de combinarlo con [`with_shrink()`](Self::with_shrink) porque con un
    /// tamaño en porcentaje, forzar `ItemShrink::Is0` sólo es seguro si el contenedor no tiene
    /// [`Gap`](super::Gap)).
    pub fn with_size(mut self, size: ItemSize) -> Self {
        self.size = self.size.set(size);
        self
    }

    /// Establece el tamaño como una fracción del contenedor (`flex-basis`), a partir del punto de
    /// corte indicado.
    pub fn with_size_at(mut self, bp: Breakpoint, size: ItemSize) -> Self {
        self.size = self.size.set_at(bp, size);
        self
    }

    /// Establece el desplazamiento respecto al inicio del contenedor (`margin-inline-start`).
    /// [`push_end()`](Self::push_end) fija este mismo campo a [`ItemOffset::Auto`]; combinar los
    /// dos deja el que se aplique en último lugar.
    pub fn with_offset(mut self, offset: ItemOffset) -> Self {
        self.offset = self.offset.set(offset);
        self
    }

    /// Establece el desplazamiento respecto al inicio del contenedor (`margin-inline-start`), a
    /// partir del punto de corte indicado.
    pub fn with_offset_at(mut self, bp: Breakpoint, offset: ItemOffset) -> Self {
        self.offset = self.offset.set_at(bp, offset);
        self
    }
}

impl FlexItem {
    /// Combina esta configuración con otra `FlexItem`, campo a campo.
    ///
    /// La fusión llega al nivel de cada punto de corte; donde `item` tenga un valor establecido,
    /// sustituye al de `self` y donde no lo tenga, se conserva el que ya hubiera. Por eso un `item`
    /// que sólo establezca `with_size_at(Breakpoint::Lg, ...)` no borra el `with_size()` base que
    /// `self` ya tuviera, sólo sustituye la entrada de ese punto de corte.
    ///
    /// Es el método que usa [`Props::with_prop()`](crate::html::props::Props::with_prop) para que
    /// sucesivas [`PropsOp::FlexItem`](crate::html::props::PropsOp::FlexItem) sobre el mismo
    /// componente vayan completando campos concretos sin repetir los ya establecidos, en vez de
    /// partir de cero en cada llamada.
    pub fn merge(mut self, item: FlexItem) -> Self {
        self.grow = self.grow.merge(item.grow);
        self.shrink = self.shrink.merge(item.shrink);
        self.align_self = self.align_self.merge(item.align_self);
        self.order = self.order.merge(item.order);
        self.size = self.size.merge(item.size);
        self.offset = self.offset.merge(item.offset);
        self
    }

    /// Aplica esta configuración como clases de utilidad responsive en el [`Context`], igual que
    /// [`Flex::apply()`](super::Flex::apply): cada faceta con valor añade una declaración de
    /// estilo (por punto de corte, si se ha establecido alguno) y su propia clase. Un campo sin
    /// ningún valor establecido, o con un valor cuya variante es la "por defecto" del propio enum
    /// (p. ej. `ItemGrow::Default`), no añade nada.
    ///
    /// Las clases generadas se añaden a `classes`, separadas con un espacio de las que ya hubiera,
    /// para poder compartir un único acumulador con [`Flex::apply()`](super::Flex::apply) sin
    /// cadenas intermedias.
    #[rustfmt::skip]
    pub(crate) fn apply(self, cx: &mut Context, classes: &mut String) {
        use crate::html::responsive::{apply, responsive_class, styles, value_to_token};

        apply!(cx, classes, self.grow, "_flex-item-grow_", "flex-grow");
        apply!(cx, classes, self.shrink, "_flex-item-shrink_", "flex-shrink");
        apply!(cx, classes, self.align_self, "_flex-item-align_", "align-self");
        apply!(cx, classes, self.order, "_flex-item-order_", "order");
        apply!(cx, classes, self.size, "_flex-item-basis_", "flex-basis", val);
        apply!(cx, classes, self.offset, "_flex-item-offset_", "margin-inline-start", val);
    }
}

impl From<FlexItem> for PropsOp {
    fn from(item: FlexItem) -> Self {
        Self::flex_item(item)
    }
}
