use crate::prelude::*;

/// Componente para presentar un **listado**: botones de acción, mensajes, tabla y paginador.
///
/// Reúne las piezas de cualquier pantalla de listado y las pinta siempre en el mismo orden, sea
/// cual sea el orden en que se añadan:
///
/// ```html
/// <div class="list-view" id="..." data-summary="...">
///   <div class="list-view-actions">...</div>  <!-- botones, si los hay -->
///   ...                                       <!-- mensajes del contexto, si los hay -->
///   <table class="table">...</table>
///   <nav class="pager">...</nav>              <!-- si hay más de una página -->
/// </div>
/// ```
///
/// Los mensajes son los que haya en el contexto al renderizar el listado (ver [`Messages`]), así
/// que un error o un aviso empujado antes, por ejemplo al rechazar un borrado, aparece junto a la
/// tabla también cuando se actualiza sólo el listado.
///
/// El listado lleva además un resumen del número de resultados (atributo `data-summary`, p. ej.
/// "23 resultados"): el total del paginador o, sin él, las filas de la tabla. Si JavaScript está
/// activo, cuando el listado se actualiza sin recargar la página, al filtrar, ordenar o cambiar de
/// página, el resumen se anuncia a los lectores de pantalla.
///
/// No sabe nada de HTMX. Para actualizar el listado sin recargar la página, se le da un
/// identificador con [`with_id()`](Self::with_id) y se apuntan a él los enlaces de la tabla y del
/// paginador. Los filtros y la búsqueda van **fuera** del listado, en la propia página: si se
/// reemplazaran con él, el campo de búsqueda perdería el foco en cada pulsación.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let list = ListView::new()
///     .with_id("item-list")
///     .with_action(
///         Button::anchor(Lc::n("New item"), "/items/new")
///             .with_style(button::Style::Solid(Intent::Primary)),
///     )
///     .with_table(
///         Table::new()
///             .with_column(Lc::n("Name"))
///             .with_empty(Lc::n("There are no items.")),
///     )
///     .with_pager(
///         Pager::new()
///             .with_base_path("/items")
///             .with_current_page(1)
///             .with_items_per_page(20)
///             .with_total_items(97),
///     );
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct ListView {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve los botones de acción del listado.
    actions: Children,
    /// Devuelve la tabla del listado.
    table: Embed<Table>,
    /// Devuelve el paginador del listado.
    pager: Embed<Pager>,
}

#[async_trait]
impl Component for ListView {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes("list-view"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Un script del componente anuncia el resumen cuando el listado se actualiza sin recargar
        // la página (al cargarla no anuncia nada).
        cx.alter_assets(
            JavaScript::defer("/pagetop/js/list-view.min.js").with_version(PAGETOP_VERSION),
        );
        let count = match self.pager().get() {
            Some(pager) => pager.total_items(),
            None => self
                .table()
                .get()
                .map_or(0, |table| table.rows().len() as u64),
        };
        let summary = Lc::l("list_view_summary")
            .with_number("count", u32::try_from(count).unwrap_or(u32::MAX))
            .lookup(cx);
        let mut actions = Flex::new()
            .with_prop(PropsOp::add_classes("list-view-actions"))
            .with_wrap(flex::Behavior::Wrap)
            .with_align(align::Items::Center)
            .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)));
        for action in self.actions().iter() {
            actions.alter_child(action.clone());
        }
        Ok(html! {
            div (self.props().unpack(cx)) data-summary=[summary] {
                (actions.render(cx).await)
                (Messages::new().render(cx).await)
                (self.table().render(cx).await)
                (self.pager().render(cx).await)
            }
        })
    }
}

#[builder_impl]
impl ListView {
    // **< ListView BUILDER >***********************************************************************

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

    /// Añade un botón de acción, o cualquier otro componente, sobre la tabla; habitualmente el de
    /// crear un elemento nuevo.
    pub fn with_action(mut self, op: impl Into<ChildOp>) -> Self {
        self.actions.alter_child(op);
        self
    }

    /// Establece la tabla del listado.
    pub fn with_table(mut self, table: Table) -> Self {
        self.table.alter_component(table);
        self
    }

    /// Establece el paginador del listado.
    pub fn with_pager(mut self, pager: Pager) -> Self {
        self.pager.alter_component(pager);
        self
    }
}
