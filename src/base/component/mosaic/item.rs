use crate::prelude::*;

// **< Span >***************************************************************************************

/// Columnas que ocupa un bloque de un [`Mosaic`](super::Mosaic) (ver [`Item::with_span()`]).
///
/// Se convierte desde un número, así que `with_span(2)` equivale a `with_span(Span::Columns(2))`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Span {
    /// Un número de columnas, como mucho las que tenga el mosaico.
    Columns(u8),
    /// Todas las columnas del mosaico, sean las que sean.
    Full,
}

impl From<u8> for Span {
    fn from(columns: u8) -> Self {
        Span::Columns(columns)
    }
}

// **< Item >***************************************************************************************

/// Bloque de un [`Mosaic`](super::Mosaic) con ajustes propios.
///
/// El mosaico envuelve cada hijo en su propio bloque (`div.mosaic-item`). Si el hijo es un
/// `mosaic::Item`, lo usa como bloque en lugar de crear otro, así que sus ajustes se aplican al
/// bloque: puede ocupar varias columnas o todas ([`with_span()`](Self::with_span)) o quedar fijado
/// a una ([`with_column()`](Self::with_column)), y los dos ajustes se pueden cambiar a partir de
/// cada punto de corte del tema ([`with_span_at()`](Self::with_span_at) y
/// [`with_column_at()`](Self::with_column_at)), como las columnas del propio mosaico.
///
/// Un bloque nunca ocupa más columnas de las que tenga el mosaico en cada momento: uno de dos
/// columnas en un mosaico que pasa de tres a dos ocupa todo el ancho, y en una sola columna, una.
/// Para mantener una proporción, se fijan sus columnas en cada punto de corte. Va al tramo de
/// columnas contiguas que lo deja más arriba y, como debe quedar debajo del bloque más bajo de ese
/// tramo, puede dejar espacio libre bajo las columnas más cortas.
///
/// Un bloque fijado se coloca antes que los demás, en lo alto de su columna, y el resto se reparte
/// a su alrededor. Como se ve antes que los bloques que lo preceden, conviene reservarlo para
/// piezas que no formen parte de la secuencia de lectura (una cabecera, un aviso destacado).
///
/// Todo esto lo hace el JavaScript del mosaico. Sin él, y al imprimir, el bloque ocupa una columna
/// en su orden, como los demás, salvo que ocupe todas ([`Span::Full`]), que se respeta también
/// entonces. Si no tiene contenido que mostrar, no ocupa sitio.
///
/// # Ejemplo
///
/// Una cabecera a todo el ancho, un bloque destacado de una columna en móviles y dos desde `Xl`, y
/// un aviso fijado en lo alto de la última columna desde `Md`:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let mosaic = Mosaic::new()
///     .with_child(
///         mosaic::Item::new()
///             .with_span(mosaic::Span::Full)
///             .with_child(Block::new().with_title(Lc::n("Today"))),
///     )
///     .with_child(
///         mosaic::Item::new()
///             .with_span_at(Breakpoint::Xl, 2)
///             .with_child(Block::new().with_title(Lc::n("Highlights"))),
///     )
///     .with_child(Block::new().with_title(Lc::n("News")))
///     .with_child(
///         mosaic::Item::new()
///             .with_column_at(Breakpoint::Md, -1)
///             .with_child(Block::new().with_title(Lc::n("Notice"))),
///     );
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Item {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del bloque.
    props: Props,
    /// Devuelve las columnas que ocupa el bloque por punto de corte, si se han fijado (si no, una).
    span: Responsive<Span>,
    /// Devuelve la columna a la que está fijado el bloque por punto de corte, si se ha fijado
    /// (contando desde 1 por el principio, o desde -1 por el final; 0 lo reparte con los demás).
    column: Responsive<i8>,
    /// Devuelve la lista de componentes hijo del bloque.
    children: Children,
}

#[async_trait]
impl Component for Item {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes("mosaic-item"));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let output = self.children().render(cx).await;
        if output.is_empty() {
            return Ok(html! {});
        }
        let mut classes = String::new();
        self.layout_classes(cx, &mut classes);
        Ok(html! {
            div (self.props().unpack_with_classes(cx, classes)) { (output) }
        })
    }
}

#[builder_impl]
impl Item {
    // **< Item BUILDER >***************************************************************************

    /// Establece el identificador único del bloque; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del bloque.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece las columnas que ocupa el bloque, sin punto de corte: un número (por defecto 1,
    /// debe ser al menos 1) o todas ([`Span::Full`]). Se aplica siempre, salvo donde lo cambie
    /// [`with_span_at()`](Self::with_span_at).
    pub fn with_span(mut self, span: impl Into<Span>) -> Self {
        let span = span.into();
        debug_assert!(
            span != Span::Columns(0),
            "A mosaic item needs at least one column"
        );
        self.span = self.span.set(span);
        self
    }

    /// Establece las columnas que ocupa el bloque a partir del punto de corte indicado.
    pub fn with_span_at(mut self, bp: Breakpoint, span: impl Into<Span>) -> Self {
        let span = span.into();
        debug_assert!(
            span != Span::Columns(0),
            "A mosaic item needs at least one column"
        );
        self.span = self.span.set_at(bp, span);
        self
    }

    /// Fija el bloque a una columna, sin punto de corte, contando desde 1 por el principio o desde
    /// -1 por el final (-1 es la última, -2 la penúltima...). Si el mosaico tiene menos columnas,
    /// queda en la más cercana, sin salirse. Con 0, el bloque se reparte con los demás, como por
    /// defecto. Se aplica siempre, salvo donde lo cambie
    /// [`with_column_at()`](Self::with_column_at).
    pub fn with_column(mut self, column: i8) -> Self {
        self.column = self.column.set(column);
        self
    }

    /// Fija el bloque a una columna a partir del punto de corte indicado (con 0, deja de estarlo).
    pub fn with_column_at(mut self, bp: Breakpoint, column: i8) -> Self {
        self.column = self.column.set_at(bp, column);
        self
    }

    /// Añade un nuevo componente al bloque o modifica la lista de componentes (`children`) con una
    /// operación [`ChildOp`].
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}

impl Item {
    // **< Item PRIVATE >***************************************************************************

    // Clases CSS de las columnas que ocupa el bloque y de la columna a la que está fijado, en cada
    // punto de corte. Dan valor a `--mosaic-span` y `--mosaic-column`, que lee el script del
    // mosaico. Las de todas las columnas añaden `column-span: all`, que respetan las columnas CSS
    // sin JavaScript y al imprimir; las de un número de columnas lo anulan, por si una de un punto
    // de corte menor lo hubiera puesto.
    fn layout_classes(&self, cx: &mut Context, classes: &mut String) {
        use crate::html::responsive::responsive_class;

        for (bp, span) in self.span().by_breakpoint() {
            let (token, column_span) = match span {
                Span::Columns(0) => continue,
                Span::Columns(n) => (n.to_string(), "none"),
                Span::Full => ("full".to_owned(), "all"),
            };
            let entry = bp.resolved(cx);
            let class = responsive_class!("_mosaic-span_", &token, entry);
            let styles = vec![
                ("--mosaic-span", token),
                ("column-span", column_span.to_owned()),
            ];
            add_class(cx, classes, entry, class, styles);
        }
        for (bp, column) in self.column().by_breakpoint() {
            let token = column.to_string();
            let entry = bp.resolved(cx);
            let class = responsive_class!("_mosaic-column_", &token, entry);
            add_class(cx, classes, entry, class, vec![("--mosaic-column", token)]);
        }
    }
}

// Añade la clase a `classes`, separada con un espacio de las que ya hubiera, y registra sus estilos
// en el punto de corte resuelto.
fn add_class(
    cx: &mut Context,
    classes: &mut String,
    entry: Option<BreakpointEntry>,
    class: String,
    styles: Vec<(&'static str, String)>,
) {
    if !classes.is_empty() {
        classes.push(' ');
    }
    classes.push_str(&class);
    cx.alter_assets(AssetsOp::add_responsive_styles(
        entry.map(|e| e.breakpoint),
        class,
        styles,
    ));
}
