use crate::core::component::Context;
use crate::core::theme::{Breakpoint, Responsive};
use crate::html::PropsOp;
use crate::html::align;
use crate::html::grid::{ItemJustify, ItemPlacement};
use crate::{AutoDefault, Getters, builder_impl, util};

/// Configuración de un elemento como ítem de un contenedor CSS Grid.
///
/// A diferencia del componente [`Grid`], que define un contenedor que aplica CSS Grid para el
/// posicionamiento de sus componentes hijo, `GridItem` se aplica sobre un único elemento en
/// relación con la rejilla del contenedor padre. Usa el método `with_prop()` que suele exponer
/// cualquier componente, y acepta `GridItem` directamente gracias a su `From` hacia [`PropsOp`].
///
/// No tiene un builder dedicado en ningún componente. De hecho, no tendría sentido porque cualquier
/// componente puede acabar siendo hijo de un contenedor Grid, y ninguno debería necesitar un campo
/// propio para esto.
///
/// `GridItem` actúa sobre propiedades del elemento para configurar su posición en columna y en
/// fila ([`ItemPlacement`]), alineación individual en el eje de columnas ([`ItemJustify`]) y en el
/// eje de filas ([`align::ItemSelf`]).
///
/// Un hijo de `Grid` sin ningún `GridItem` aplicado participa igualmente en la colocación
/// automática, sólo que sin ninguna posición ni alineación propia.
///
/// [`Grid`]: crate::base::component::Grid
/// [`align::ItemSelf`]: crate::html::align::ItemSelf
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// // Ocupa las tres primeras columnas de la rejilla de su `Grid` padre.
/// let banner = Container::new()
///     .with_prop(GridItem::new().with_column(grid::ItemPlacement::Span(3)));
///
/// // Empieza en la columna 2 y termina antes de la 4, alineado al final de su fila.
/// let sidebar = Container::new().with_prop(
///     GridItem::new()
///         .with_column(grid::ItemPlacement::Range(2, 4))
///         .with_align_self(align::ItemSelf::End),
/// );
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq, Getters)]
pub struct GridItem {
    /// Devuelve la posición en columna, por punto de corte.
    #[getters(copy)]
    column: Responsive<ItemPlacement>,
    /// Devuelve la posición en fila, por punto de corte.
    #[getters(copy)]
    row: Responsive<ItemPlacement>,
    /// Devuelve la alineación individual en el eje de columnas, por punto de corte.
    #[getters(copy)]
    justify_self: Responsive<ItemJustify>,
    /// Devuelve la alineación individual en el eje de filas, por punto de corte.
    #[getters(copy)]
    align_self: Responsive<align::ItemSelf>,
}

#[builder_impl]
impl GridItem {
    /// Crea una configuración de ítem con todos los valores por defecto.
    pub fn new() -> Self {
        Self::default()
    }

    // **< GridItem BUILDER >***********************************************************************

    /// Establece la posición en columna.
    pub fn with_column(mut self, placement: ItemPlacement) -> Self {
        self.column = self.column.set(placement);
        self
    }

    /// Establece la posición en columna, a partir del punto de corte indicado.
    pub fn with_column_at(mut self, bp: Breakpoint, placement: ItemPlacement) -> Self {
        self.column = self.column.set_at(bp, placement);
        self
    }

    /// Establece la posición en fila.
    pub fn with_row(mut self, placement: ItemPlacement) -> Self {
        self.row = self.row.set(placement);
        self
    }

    /// Establece la posición en fila, a partir del punto de corte indicado.
    pub fn with_row_at(mut self, bp: Breakpoint, placement: ItemPlacement) -> Self {
        self.row = self.row.set_at(bp, placement);
        self
    }

    /// Establece la alineación individual en el eje de columnas.
    pub fn with_justify_self(mut self, justify_self: ItemJustify) -> Self {
        self.justify_self = self.justify_self.set(justify_self);
        self
    }

    /// Establece la alineación individual en el eje de columnas, a partir del punto de corte
    /// indicado.
    pub fn with_justify_self_at(mut self, bp: Breakpoint, justify_self: ItemJustify) -> Self {
        self.justify_self = self.justify_self.set_at(bp, justify_self);
        self
    }

    /// Establece la alineación individual en el eje de filas.
    pub fn with_align_self(mut self, align_self: align::ItemSelf) -> Self {
        self.align_self = self.align_self.set(align_self);
        self
    }

    /// Establece la alineación individual en el eje de filas, a partir del punto de corte
    /// indicado.
    pub fn with_align_self_at(mut self, bp: Breakpoint, align_self: align::ItemSelf) -> Self {
        self.align_self = self.align_self.set_at(bp, align_self);
        self
    }
}

impl GridItem {
    /// Combina esta configuración con otra `GridItem`, campo a campo.
    ///
    /// La fusión llega al nivel de cada punto de corte; donde `item` tenga un valor establecido,
    /// sustituye al de `self` y donde no lo tenga, se conserva el que ya hubiera. Por eso un `item`
    /// que sólo establezca `with_column_at(Breakpoint::Lg, ...)` no borra el `with_column()` base
    /// que `self` ya tuviera, sólo sustituye la entrada de ese punto de corte.
    ///
    /// Es el método que usa [`Props::with_prop()`] para que sucesivas [`PropsOp::GridItem`] sobre
    /// el mismo componente vayan completando campos concretos sin repetir los ya establecidos, en
    /// vez de partir de cero en cada llamada.
    ///
    /// [`Props::with_prop()`]: crate::html::props::Props::with_prop
    /// [`PropsOp::GridItem`]: crate::html::props::PropsOp::GridItem
    pub fn merge(mut self, item: GridItem) -> Self {
        self.column = self.column.merge(item.column);
        self.row = self.row.merge(item.row);
        self.justify_self = self.justify_self.merge(item.justify_self);
        self.align_self = self.align_self.merge(item.align_self);
        self
    }

    /// Aplica esta configuración como clases de utilidad responsive en el [`Context`], igual que
    /// hace internamente [`Grid`](crate::base::component::Grid): cada propiedad con valor añade una
    /// declaración de estilo (por punto de corte, si se ha establecido alguno) y su propia clase.
    /// Un campo sin ningún valor establecido, o con un valor cuya variante es la "por defecto" del
    /// propio enum (p. ej. `ItemJustify::Default`), no añade nada.
    ///
    /// Las clases generadas se añaden a `classes`, separadas con un espacio de las que ya hubiera,
    /// para poder compartir un único acumulador con las de `Grid` sin cadenas intermedias.
    #[rustfmt::skip]
    pub(crate) fn apply(self, cx: &mut Context, classes: &mut String) {
        use crate::html::responsive::{apply, responsive_class, styles, value_to_token};

        apply!(cx, classes, self.column, "_grid-item-column_", "grid-column", val);
        apply!(cx, classes, self.row, "_grid-item-row_", "grid-row", val);
        apply!(cx, classes, self.justify_self, "_grid-item-justify-self_", "justify-self");
        apply!(cx, classes, self.align_self, "_grid-item-align-self_", "align-self");
    }
}

impl From<GridItem> for PropsOp {
    fn from(item: GridItem) -> Self {
        Self::grid_item(item)
    }
}
