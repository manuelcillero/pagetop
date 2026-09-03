use crate::html::flex::props_item::{
    ItemAlign, ItemGrow, ItemOffset, ItemOrder, ItemShrink, ItemSize,
};
use crate::html::props::{Props, PropsOp};
use crate::{AutoDefault, Getters, builder_impl};

// **< FlexItem >***********************************************************************************

/// Configuración de un elemento como ítem de un contenedor Flexbox.
///
/// A diferencia de [`Flex`](crate::html::flex::Flex), que configura el comportamiento Flexbox
/// global de un contenedor y sus hijos como grupo, `FlexItem` configura un único elemento en
/// relación con el contenedor flex padre: crecimiento ([`ItemGrow`]), reducción ([`ItemShrink`]),
/// alineación individual ([`ItemAlign`]), orden visual ([`ItemOrder`]), ancho ([`ItemSize`]) y
/// desplazamiento ([`ItemOffset`]).
///
/// No tiene un builder dedicado en ningún componente. De hecho, no tendría sentido porque cualquier
/// componente puede acabar siendo hijo de un contenedor flex, y ninguno debería necesitar un campo
/// propio para esto. Se aplica con [`PropsOp::flex_item()`] sobre el `with_prop()` que suele
/// exponer cualquier componente.
///
/// Con [`ItemSize`] y [`ItemOffset`] se pueden modelar rejillas de columnas fijas sobre Flexbox,
/// combinando un ancho en fracción del contenedor con un desplazamiento lateral cuando se necesite.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// // Crece para ocupar el espacio sobrante, partiendo de ancho cero.
/// let title = Button::plain(Lc::n("Panel")).with_prop(PropsOp::flex_item(
///     FlexItem::new()
///         .with_grow(flex::ItemGrow::Is1)
///         .with_size(flex::ItemSize::Custom(UnitValue::Zero)),
/// ));
///
/// // Ocupa un tercio del ancho del contenedor, desplazado otro tercio desde el inicio.
/// let column = Container::new().with_prop(PropsOp::flex_item(
///     FlexItem::new()
///         .with_size(flex::ItemSize::Percent33)
///         .with_offset(flex::ItemOffset::Percent33),
/// ));
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq, Getters)]
pub struct FlexItem {
    /// Devuelve el factor de crecimiento.
    #[getters(copy)]
    grow: ItemGrow,
    /// Devuelve el factor de reducción.
    #[getters(copy)]
    shrink: ItemShrink,
    /// Devuelve la alineación individual en el eje transversal.
    #[getters(copy)]
    align_self: ItemAlign,
    /// Devuelve la posición en el orden visual.
    #[getters(copy)]
    order: ItemOrder,
    /// Devuelve el ancho como fracción del contenedor.
    #[getters(copy)]
    size: ItemSize,
    /// Devuelve el desplazamiento respecto al inicio del contenedor.
    #[getters(copy)]
    offset: ItemOffset,
}

#[builder_impl]
impl FlexItem {
    /// Crea una configuración de ítem con todos los valores por defecto.
    pub fn new() -> Self {
        Self::default()
    }

    // **< FlexItem BUILDER >***********************************************************************

    /// Establece el factor de crecimiento.
    pub fn with_grow(mut self, grow: ItemGrow) -> Self {
        self.grow = grow;
        self
    }

    /// Establece el factor de reducción.
    pub fn with_shrink(mut self, shrink: ItemShrink) -> Self {
        self.shrink = shrink;
        self
    }

    /// Establece la alineación individual en el eje transversal.
    pub fn with_align_self(mut self, align_self: ItemAlign) -> Self {
        self.align_self = align_self;
        self
    }

    /// Establece la posición en el orden visual.
    pub fn with_order(mut self, order: ItemOrder) -> Self {
        self.order = order;
        self
    }

    /// Establece el ancho como una fracción del contenedor (`flex-basis`). No fuerza
    /// [`ItemShrink::Is0`](super::ItemShrink::Is0) por sí solo (consulta la documentación de
    /// [`ItemSize`] antes de combinarlo con [`with_shrink()`](Self::with_shrink) porque con un
    /// tamaño en porcentaje, forzar `ItemShrink::Is0` sólo es seguro si el contenedor no tiene
    /// [`Gap`](super::Gap)).
    pub fn with_size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }

    /// Establece el desplazamiento respecto al inicio del contenedor (`margin-inline-start`). No
    /// tiene relación con [`push_end()`](Self::push_end) aunque aplican la misma propiedad CSS para
    /// casos de uso distintos.
    pub fn with_offset(mut self, offset: ItemOffset) -> Self {
        self.offset = offset;
        self
    }
}

impl FlexItem {
    // Aplica esta configuración a un Props como declaraciones de estilo en línea.
    pub(crate) fn apply_to(self, props: &mut Props) {
        for (property, value) in [
            ("flex-grow", self.grow.value()),
            ("flex-shrink", self.shrink.value()),
            ("align-self", self.align_self.value()),
            ("order", self.order.value()),
            ("flex-basis", self.size.value()),
            ("margin-inline-start", self.offset.value()),
        ] {
            props.alter_prop(PropsOp::add_style(property, value));
        }
    }

    /// Separa un elemento (y los que le sigan en el mismo eje principal) del resto, empujándolo
    /// hacia el extremo final de un contenedor flex.
    ///
    /// Se resuelve siempre como margen inicial automático (`margin-inline-start: auto`) en línea,
    /// igual que el resto de facetas de `FlexItem`. Es el mecanismo estándar de Flexbox para, por
    /// ejemplo, separar dos menús dentro de una misma [`Navbar`](crate::base::component::Navbar)
    /// -- uno pegado al inicio, el siguiente empujado al final -- sin que el contenedor necesite
    /// conocer ninguna distinción entre sus elementos.
    ///
    /// No forma parte de los campos de `FlexItem` (no se combina con `grow`/`shrink`/`align_self`/
    /// `order`/`size`/`offset` en una misma llamada): es una función asociada independiente porque
    /// resuelve un caso de uso completo por sí sola, con una sola línea, y vive aquí -- en vez de
    /// como función suelta del módulo `flex` -- para dejar claro que es una operación de **ítem**,
    /// no de contenedor.
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
    pub fn push_end() -> PropsOp {
        PropsOp::add_style("margin-inline-start", "auto")
    }
}

impl From<FlexItem> for PropsOp {
    fn from(item: FlexItem) -> Self {
        Self::flex_item(item)
    }
}
