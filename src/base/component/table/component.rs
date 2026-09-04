use crate::prelude::*;

/// Componente para representar **tablas de datos**.
///
/// `Table` resuelve la estructura HTML de cualquier listado tabular (`<table>`, `<thead>`,
/// `<tbody>` y la fila para una tabla "sin resultados") y deja el resto en manos de quien lo usa:
/// qué contiene cada celda, qué columnas son ordenables y hacia dónde apuntan sus enlaces, y la
/// paginación (que se compone aparte, normalmente junto a `Table` dentro de un mismo contenedor).
///
/// # Clases CSS
///
/// - `.table-responsive`: envuelve `<table>` para admitir scroll horizontal sólo de la tabla si no
///   cabe en el ancho disponible.
/// - `.table`: clase base del elemento `<table>`.
/// - `.table-sort`: presente en el enlace de una cabecera ordenable.
/// - `.table-sort-asc` / `.table-sort-desc`: añadidas junto a `.table-sort` cuando esa columna es
///   la que determina el orden vigente. El tema activo puede usarlas para dibujar el indicador
///   visual (flecha, icono...) mediante CSS; `Table` no incluye ningún glifo por sí misma.
/// - `.table-empty`: clase de la celda con el mensaje mostrado cuando no hay filas.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let current_sort = "username";
/// let current_dir = SortDir::Asc;
///
/// // Definición de la tabla y cabeceras.
/// let mut table = Table::new()
///     .with_column(
///         table::Column::new(Lc::n("User")).with_sort(
///             table::SortLink::new("/admin/users?sort=username")
///                 .with_dir((current_sort == "username").then_some(current_dir)),
///         ),
///     )
///     .with_column(Lc::n("Email"))
///     // Con `None` en vez de un `Lc`, se desactiva el mensaje predeterminado.
///     .with_empty(Lc::n("No users to display."));
///
/// // Contenido de la tabla.
/// for (username, email) in [("julia", "julia@example.com"), ("Fran", "fran@example.com")] {
///     table.alter_row(
///         table::Row::new()
///             // Ambos campos son del mismo tipo. Uno lo asignamos en un componente `Html`.
///             .with_cell(table::Cell::new(Html::with(move |_| html! { (username) })))
///             // Y el otro como `&str`, que se convierte directamente en `Cell`.
///             .with_cell(email),
///     );
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Table {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve las columnas de la tabla, en orden de aparición.
    columns: Vec<table::Column>,
    /// Devuelve las filas de datos de la tabla, en orden de aparición.
    rows: Vec<table::Row>,
    /// Devuelve el mensaje mostrado cuando no hay filas, o `None` si está desactivado.
    #[default(Some(Lc::l("table_empty")))]
    empty: Option<Lc>,
}

#[async_trait]
impl Component for Table {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes("table"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let colspan = self.columns().len().max(1).to_string();

        Ok(html! {
            div.table-responsive {
                table (self.props().unpack(cx)) {
                    @if !self.columns().is_empty() {
                        thead {
                            tr {
                                @for column in self.columns() {
                                    (column.render_header(cx))
                                }
                            }
                        }
                    }
                    @if !self.rows().is_empty() {
                        tbody {
                            @for row in self.rows() {
                                tr (row.props().unpack(cx)) {
                                    @for cell in row.cells() {
                                        td (cell.props().unpack(cx)) { (cell.children().render(cx).await) }
                                    }
                                }
                            }
                        }
                    } @else if let Some(empty) = self
                        .empty()
                        .and_then(|e| e.lookup(cx))
                        .filter(|s| !s.is_empty())
                    {
                        tbody {
                            tr {
                                td.table-empty colspan=(colspan) { (empty) }
                            }
                        }
                    }
                }
            }
        })
    }
}

#[builder_impl]
impl Table {
    // **< Table BUILDER >**************************************************************************

    /// Establece el identificador único de la tabla.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS o atributos HTML de la tabla.
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Añade una columna al final de la cabecera.
    ///
    /// Acepta directamente un `&str`, un `String` o un [`Lc`] (que equivalen a
    /// `table::Column::new(...)` con el texto indicado), o un [`table::Column`] ya construido (por
    /// ejemplo para asignarle clases, atributos propios o un enlace de ordenación con
    /// `with_sort()`).
    pub fn with_column(mut self, column: impl Into<table::Column>) -> Self {
        self.columns.push(column.into());
        self
    }

    /// Añade una fila de datos al final de la tabla.
    pub fn with_row(mut self, row: table::Row) -> Self {
        self.rows.push(row);
        self
    }

    /// Sustituye el mensaje mostrado cuando no hay filas, o lo desactiva con `None`: en ese caso,
    /// una tabla sin filas no muestra ninguna fila de reemplazo; sólo se renderizan `<table>` y, si
    /// hay columnas, `<thead>`.
    ///
    /// Ese mismo resultado se obtiene también si la traducción no resuelve a ningún texto (por
    /// ejemplo, con `Lc::n("")`).
    pub fn with_empty(mut self, empty: impl Into<Option<Lc>>) -> Self {
        self.empty = empty.into();
        self
    }
}
