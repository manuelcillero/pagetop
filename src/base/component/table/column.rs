use crate::prelude::*;

/// Cabecera de columna (`<th>`) de una [`Table`].
///
/// El contenido (`label`) es un [`Lc`]. Si la columna debe permitir ordenar la tabla al pulsarla,
/// añade un [`table::SortLink`] con [`with_sort()`](Self::with_sort) para que `Table` envuelva la
/// etiqueta en un enlace y añada el `aria-sort` y las clases CSS correspondientes.
///
/// Un `&str`, un `String` o un [`Lc`] se convierten directamente en `Column`; para estos tipos no
/// sería necesario llamar a [`Column::new()`] de forma explícita.
///
/// ```rust
/// use pagetop::locale::Lc;
/// use pagetop::base::component::table::Column;
///
/// let column: Column = Lc::n("User").into();
/// assert_eq!(column.label().get(), Some("User".to_string()));
/// assert!(column.sort().is_none());
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Column {
    /// Devuelve identificador, clases CSS y atributos HTML de la celda de cabecera (`<th>`).
    props: Props,
    /// Devuelve el contenido de la cabecera.
    label: Lc,
    /// Devuelve el enlace de ordenación de la columna, si es ordenable.
    sort: Option<table::SortLink>,
}

impl Column {
    /// Crea una cabecera con el texto localizado indicado.
    pub fn new(label: Lc) -> Self {
        Self {
            label,
            ..Default::default()
        }
    }

    // **< Column BUILDER >*************************************************************************

    /// Establece el identificador único de la celda de cabecera.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS o atributos HTML de la columna.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el número de columnas que ocupa la cabecera (atributo `colspan`).
    ///
    /// Con `1` (el valor por defecto de HTML) elimina el atributo en vez de fijarlo.
    #[builder_fn]
    pub fn with_colspan(mut self, span: u8) -> Self {
        self.props.alter_prop(if span == 1 {
            PropsOp::remove("colspan")
        } else {
            PropsOp::set("colspan", span.to_string())
        });
        self
    }

    /// Establece el número de filas que ocupa la cabecera (atributo `rowspan`).
    ///
    /// Con `1` (el valor por defecto de HTML) elimina el atributo en vez de fijarlo.
    #[builder_fn]
    pub fn with_rowspan(mut self, span: u8) -> Self {
        self.props.alter_prop(if span == 1 {
            PropsOp::remove("rowspan")
        } else {
            PropsOp::set("rowspan", span.to_string())
        });
        self
    }

    /// Convierte la columna en ordenable con el enlace indicado, o la vuelve no ordenable con
    /// `None`.
    #[builder_fn]
    pub fn with_sort(mut self, sort: impl Into<Option<table::SortLink>>) -> Self {
        self.sort = sort.into();
        self
    }

    // Traduce la etiqueta y, si la columna es ordenable, la envuelve en su enlace con `aria-sort`
    // y las clases `table-sort*` ya resueltas por `table::SortLink`. Sólo lo usa `Table` al
    // renderizar.
    pub(super) fn render_header(&self, cx: &Context) -> Markup {
        let label = self.label().using(cx);

        let Some(sort) = self.sort() else {
            return html! { th (self.props()) scope="col" { (label) } };
        };
        let (aria_sort, link_props) = sort.header_attrs();

        html! {
            th (self.props()) scope="col" aria-sort=(aria_sort) {
                a href=[sort.href().as_deref()] (link_props) { (label) }
            }
        }
    }
}

impl From<&str> for Column {
    /// Convierte un `&str` en una cabecera de texto literal; equivale a `Column::new(Lc::n(text))`.
    fn from(text: &str) -> Self {
        Column::new(Lc::n(text.to_string()))
    }
}

impl From<String> for Column {
    /// Convierte un `String` en una cabecera de texto literal; equivale a
    /// `Column::new(Lc::n(text))`.
    fn from(text: String) -> Self {
        Column::new(Lc::n(text))
    }
}

impl From<Lc> for Column {
    /// Convierte un [`Lc`] en una cabecera de texto traducible; equivale a `Column::new(label)`.
    fn from(label: Lc) -> Self {
        Column::new(label)
    }
}
