use crate::html::align::{Content, Gap, Items};
use crate::html::grid::{AutoFlow, AxisTrack, ContentJustify, DefaultJustify, Tracks};
use crate::prelude::*;

// **< DisplayGrid >********************************************************************************

// Posicionamiento CSS Grid del contenedor `Grid`. La API pública sólo expone los constructores
// `Grid::new()`, `Grid::at()`, `Grid::inline()` e `Grid::inline_at()`, nunca la variante en sí.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
enum DisplayGrid {
    #[default]
    Always,
    AlwaysInline,
    At(Breakpoint),
    InlineAt(Breakpoint),
}

// **< Grid >***************************************************************************************

/// Componente que crea una **rejilla CSS Grid** para posicionar componentes.
///
/// Es el único componente de PageTop que ofrece posicionamiento CSS Grid (con el mismo criterio que
/// [`Flex`] para Flexbox). Sus hijos pueden ser **cualquier componente**, no sólo otro `Grid`. Para
/// colocarlos dentro de la rejilla (columna, fila, alineación individual) se usa [`grid::GridItem`]
/// vía `with_prop()`, exactamente igual que en cualquier otro contenedor. `GridItem` es una
/// característica de [`PropsOp`], no exige ningún envoltorio propio.
///
/// Se aplica con clases CSS generadas dinámicamente (`display`, `grid-template-columns`,
/// `grid-template-rows`, `grid-auto-columns`, `grid-auto-rows`, `grid-auto-flow`, `justify-items`,
/// `align-items`, `justify-content`, `align-content`, `gap`, `row-gap`, `column-gap`), registradas
/// vía [`AssetsOp::add_responsive_style()`] en [`ResponsiveStyles`] y renderizadas como reglas en
/// el `<head>` del documento. Son propiedades nativas que no requieren interpretación por parte de
/// los temas, siempre funcionan igual, sin una sola línea de CSS ni de código específico.
///
/// `align_items`/`align_content`/`gap` usan tipos de [`pagetop::html::align`] ([`align::Items`],
/// [`align::Content`], [`align::Gap`]), compartidos con [`Flex`], mismo CSS y mismo catálogo de
/// valores en los dos. Ver la documentación de [`align`](crate::html::align) para el criterio
/// completo.
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// [`AssetsOp::add_responsive_style()`]: crate::core::component::AssetsOp::add_responsive_style
/// [`pagetop::html::align`]: crate::html::align
/// [`grid::GridItem`]: crate::html::grid::GridItem
/// [`ResponsiveStyles`]: crate::html::ResponsiveStyles
/// [`Flex`]: crate::base::component::Flex
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let cards = Grid::new()
///     .with_columns(grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0)))
///     .with_gap(align::Gap::Both(UnitValue::RelRem(1.0)))
///     .with_child(Container::new())
///     .with_child(Container::new())
///     .with_child(Container::new());
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Grid {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    // Determina si esta configuración debe aplicarse (y con qué variante de `display`) o si `Grid`
    // no está en absoluto configurado. `None` es el estado real de ausencia: lo que tiene un
    // componente que nunca ha llamado a ninguno de sus constructores explícitos. Sin getter
    // público; `new()`, `at()`, `inline()` e `inline_at()` son la única forma de activarlo.
    #[getters(skip)]
    display: Option<DisplayGrid>,
    /// Devuelve las pistas de columna (`grid-template-columns`), por punto de corte.
    #[getters(copy)]
    columns: Responsive<Tracks>,
    /// Devuelve las pistas de fila (`grid-template-rows`), por punto de corte.
    #[getters(copy)]
    rows: Responsive<Tracks>,
    /// Devuelve el tamaño de las columnas generadas implícitamente (`grid-auto-columns`), por
    /// punto de corte.
    #[getters(copy)]
    auto_columns: Responsive<AxisTrack>,
    /// Devuelve el tamaño de las filas generadas implícitamente (`grid-auto-rows`), por punto de
    /// corte.
    #[getters(copy)]
    auto_rows: Responsive<AxisTrack>,
    /// Devuelve el algoritmo de colocación automática, por punto de corte.
    #[getters(copy)]
    auto_flow: Responsive<AutoFlow>,
    /// Devuelve la alineación de los elementos en el eje de columnas, por punto de corte.
    #[getters(copy)]
    justify_items: Responsive<DefaultJustify>,
    /// Devuelve la alineación de los elementos en el eje de filas, por punto de corte.
    #[getters(copy)]
    align_items: Responsive<Items>,
    /// Devuelve la alineación de las pistas en el eje de columnas, por punto de corte.
    #[getters(copy)]
    justify_content: Responsive<ContentJustify>,
    /// Devuelve la alineación de las pistas en el eje de filas, por punto de corte.
    #[getters(copy)]
    align_content: Responsive<Content>,
    /// Devuelve el espaciado entre pistas, por punto de corte.
    #[getters(copy)]
    gap: Responsive<Gap>,
    /// Devuelve la lista de componentes (`children`) del contenedor.
    children: Children,
}

#[async_trait]
impl Component for Grid {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let output = self.children().render(cx).await;
        if output.is_empty() {
            return Ok(html! {});
        }
        let mut classes = String::new();
        self.grid_classes(cx, &mut classes);
        let container_props = self.props().unpack_with_classes(cx, classes);
        Ok(html! { div (container_props) { (output) } })
    }
}

#[builder_impl]
impl Grid {
    /// Define una configuración con `display: grid`, sin punto de corte: se aplica siempre.
    pub fn new() -> Self {
        Self {
            display: Some(DisplayGrid::Always),
            ..Default::default()
        }
    }

    /// Define una configuración con `display: grid` que se aplica a partir del punto de corte
    /// indicado.
    pub fn at(bp: Breakpoint) -> Self {
        Self {
            display: Some(DisplayGrid::At(bp)),
            ..Default::default()
        }
    }

    /// Define una configuración con `display: inline-grid`, sin punto de corte: se aplica
    /// siempre.
    pub fn inline() -> Self {
        Self {
            display: Some(DisplayGrid::AlwaysInline),
            ..Default::default()
        }
    }

    /// Define una configuración con `display: inline-grid` que se aplica a partir del punto de
    /// corte indicado.
    pub fn inline_at(bp: Breakpoint) -> Self {
        Self {
            display: Some(DisplayGrid::InlineAt(bp)),
            ..Default::default()
        }
    }

    // **< Grid BUILDER >***************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece las pistas de columna.
    pub fn with_columns(mut self, tracks: Tracks) -> Self {
        self.columns = self.columns.set(tracks);
        self
    }

    /// Establece las pistas de columna a partir del punto de corte indicado.
    pub fn with_columns_at(mut self, bp: Breakpoint, tracks: Tracks) -> Self {
        self.columns = self.columns.set_at(bp, tracks);
        self
    }

    /// Establece las pistas de fila.
    pub fn with_rows(mut self, tracks: Tracks) -> Self {
        self.rows = self.rows.set(tracks);
        self
    }

    /// Establece las pistas de fila a partir del punto de corte indicado.
    pub fn with_rows_at(mut self, bp: Breakpoint, tracks: Tracks) -> Self {
        self.rows = self.rows.set_at(bp, tracks);
        self
    }

    /// Establece el tamaño de las columnas generadas implícitamente por la colocación automática.
    pub fn with_auto_columns(mut self, track: AxisTrack) -> Self {
        self.auto_columns = self.auto_columns.set(track);
        self
    }

    /// Establece el tamaño de las columnas generadas implícitamente, a partir del punto de corte
    /// indicado.
    pub fn with_auto_columns_at(mut self, bp: Breakpoint, track: AxisTrack) -> Self {
        self.auto_columns = self.auto_columns.set_at(bp, track);
        self
    }

    /// Establece el tamaño de las filas generadas implícitamente por la colocación automática.
    pub fn with_auto_rows(mut self, track: AxisTrack) -> Self {
        self.auto_rows = self.auto_rows.set(track);
        self
    }

    /// Establece el tamaño de las filas generadas implícitamente, a partir del punto de corte
    /// indicado.
    pub fn with_auto_rows_at(mut self, bp: Breakpoint, track: AxisTrack) -> Self {
        self.auto_rows = self.auto_rows.set_at(bp, track);
        self
    }

    /// Establece el algoritmo de colocación automática.
    pub fn with_auto_flow(mut self, auto_flow: AutoFlow) -> Self {
        self.auto_flow = self.auto_flow.set(auto_flow);
        self
    }

    /// Establece el algoritmo de colocación automática, a partir del punto de corte indicado.
    pub fn with_auto_flow_at(mut self, bp: Breakpoint, auto_flow: AutoFlow) -> Self {
        self.auto_flow = self.auto_flow.set_at(bp, auto_flow);
        self
    }

    /// Establece la alineación de los elementos en el eje de columnas.
    pub fn with_justify_items(mut self, justify_items: DefaultJustify) -> Self {
        self.justify_items = self.justify_items.set(justify_items);
        self
    }

    /// Establece la alineación de los elementos en el eje de columnas, a partir del punto de corte
    /// indicado.
    pub fn with_justify_items_at(mut self, bp: Breakpoint, justify_items: DefaultJustify) -> Self {
        self.justify_items = self.justify_items.set_at(bp, justify_items);
        self
    }

    /// Establece la alineación de los elementos en el eje de filas.
    pub fn with_align_items(mut self, align_items: Items) -> Self {
        self.align_items = self.align_items.set(align_items);
        self
    }

    /// Establece la alineación de los elementos en el eje de filas, a partir del punto de corte
    /// indicado.
    pub fn with_align_items_at(mut self, bp: Breakpoint, align_items: Items) -> Self {
        self.align_items = self.align_items.set_at(bp, align_items);
        self
    }

    /// Establece la alineación de las pistas en el eje de columnas (ver [`ContentJustify`]).
    pub fn with_justify_content(mut self, justify_content: ContentJustify) -> Self {
        self.justify_content = self.justify_content.set(justify_content);
        self
    }

    /// Establece la alineación de las pistas en el eje de columnas, a partir del punto de corte
    /// indicado.
    pub fn with_justify_content_at(
        mut self,
        bp: Breakpoint,
        justify_content: ContentJustify,
    ) -> Self {
        self.justify_content = self.justify_content.set_at(bp, justify_content);
        self
    }

    /// Establece la alineación de las pistas en el eje de filas (ver
    /// [`Content`](crate::html::align::Content)).
    pub fn with_align_content(mut self, align_content: Content) -> Self {
        self.align_content = self.align_content.set(align_content);
        self
    }

    /// Establece la alineación de las pistas en el eje de filas, a partir del punto de corte
    /// indicado.
    pub fn with_align_content_at(mut self, bp: Breakpoint, align_content: Content) -> Self {
        self.align_content = self.align_content.set_at(bp, align_content);
        self
    }

    /// Establece el espaciado entre pistas.
    pub fn with_gap(mut self, gap: Gap) -> Self {
        self.gap = self.gap.set(gap);
        self
    }

    /// Establece el espaciado entre pistas, a partir del punto de corte indicado.
    pub fn with_gap_at(mut self, bp: Breakpoint, gap: Gap) -> Self {
        self.gap = self.gap.set_at(bp, gap);
        self
    }

    /// Añade un nuevo componente al contenedor o modifica la lista de componentes (`children`) con
    /// una operación [`ChildOp`].
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}

impl Grid {
    // Calcula las clases CSS *responsive* de este contenedor Grid y las añade a `classes`,
    // separadas con un espacio de las que ya hubiera, para poder compartir un único acumulador con
    // `GridItem::apply()` (ver `Props::unpack_with_classes()`) sin cadenas intermedias.
    #[rustfmt::skip]
    fn grid_classes(&self, cx: &mut Context, classes: &mut String) {
        // Sin `display` no hay contenedor Grid: el resto de propiedades (`grid-template-columns`,
        // `gap`, etc.) no tienen ningún efecto en CSS sin `display: grid`/`inline-grid`, así que ni
        // se generan. Éste es el estado de un `Grid` que nunca ha llamado a ninguno de sus
        // constructores explícitos.
        let Some(display) = self.display else {
            return;
        };

        use crate::html::responsive::{apply, responsive_class, styles, value_to_token};

        let (prefix, value) = match display {
            DisplayGrid::Always
                | DisplayGrid::At(_) => ("_grid_", "grid"),
            DisplayGrid::AlwaysInline
                | DisplayGrid::InlineAt(_) => ("_inline-grid_", "inline-grid"),
        };
        let entry = match display {
            DisplayGrid::At(bp) | DisplayGrid::InlineAt(bp) => bp.resolved(cx),
            _ => None,
        };
        let class = match entry {
            None => prefix.into(),
            Some(entry) => util::join!(prefix, entry.name, "_").into(),
        };
        styles(cx, classes, entry, class, "display", value.into());

        apply!(cx, classes, self.columns, "_grid-columns_", "grid-template-columns", val);
        apply!(cx, classes, self.rows, "_grid-rows_", "grid-template-rows", val);
        apply!(cx, classes, self.auto_columns, "_grid-auto-columns_", "grid-auto-columns", val);
        apply!(cx, classes, self.auto_rows, "_grid-auto-rows_", "grid-auto-rows", val);
        // `val`: a diferencia de `flex::Behavior` (una sola palabra en cualquier variante),
        // `AutoFlow::RowDense`/`ColumnDense` valen "row dense"/"column dense" (con espacio), así
        // que necesitan pasar por `value_to_token()` igual que `Tracks`/`ItemPlacement`.
        apply!(cx, classes, self.auto_flow, "_grid-auto-flow_", "grid-auto-flow", val);
        apply!(cx, classes, self.justify_items, "_grid-justify-items_", "justify-items");
        apply!(cx, classes, self.align_items, "_grid-align-items_", "align-items");
        apply!(cx, classes, self.justify_content, "_grid-justify-content_", "justify-content");
        apply!(cx, classes, self.align_content, "_grid-align-content_", "align-content");
        for (bp, gap) in self.gap.by_breakpoint() {
            let entry = bp.resolved(cx);
            for (property, value) in gap.styles().into_iter().flatten() {
                // El prefijo de `gap` no es literal (depende de la propiedad), así que se compone
                // aquí en un único `join!` en vez de pasar por `responsive_class!`.
                let token = value_to_token(&value);
                let class = match entry {
                    None => util::join!("_grid-", property, "_", token, "_"),
                    Some(e) => util::join!("_grid-", property, "_", token, "_", e.name, "_"),
                };
                styles(cx, classes, entry, class.into(), property, value);
            }
        }
    }
}
