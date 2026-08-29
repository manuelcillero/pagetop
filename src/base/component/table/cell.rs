use crate::prelude::*;

/// Representa una celda de datos (`<td>`) de una [`table::Row`].
///
/// El contenido es una lista [`Children`] para admitir cualquier número de componentes.
/// [`Cell::new()`] añade un componente como contenido inicial, normalmente un [`Lc`] con texto
/// literal ([`Lc::n()`]) o traducible (con [`Lc::l()`]/[`Lc::t()`]).
///
/// Un `&str`, un `String` o un [`Lc`] se convierten directamente en `Cell`; para estos tipos no
/// sería necesario llamar a [`Cell::new()`] de forma explícita.
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let email: table::Cell = "ana@example.com".into();
/// let label: table::Cell = Lc::l("table-status").into();
/// let name = table::Cell::new(Html::with(|_| html! { "Julia" }));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Cell {
    /// Devuelve identificador, clases CSS y atributos HTML de la celda (`<td>`).
    props: Props,
    /// Devuelve la lista de componentes hijo que genera el contenido de la celda.
    children: Children,
}

#[builder_impl]
impl Cell {
    /// Crea una celda a partir del componente o texto ([`Lc`]) indicado.
    ///
    /// Para combinar varios componentes en la misma celda basta con encadenar llamadas a
    /// [`with_child()`](Cell::with_child):
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let status = table::Cell::new(Lc::n("Admin ")).with_child(Badge::labeled(Lc::n("active")));
    /// ```
    pub fn new(child: impl Into<Child>) -> Self {
        Self {
            children: Children::with(child.into()),
            ..Default::default()
        }
    }

    // **< Cell BUILDER >***************************************************************************

    /// Establece el identificador único de la celda.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS o atributos HTML de la celda.
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el número de columnas que ocupa la celda (atributo `colspan`).
    ///
    /// Con `1` (el valor por defecto de HTML) elimina el atributo en vez de fijarlo.
    pub fn with_colspan(mut self, span: u8) -> Self {
        self.props.alter_prop(if span == 1 {
            PropsOp::remove("colspan")
        } else {
            PropsOp::set("colspan", span.to_string())
        });
        self
    }

    /// Establece el número de filas que ocupa la celda (atributo `rowspan`).
    ///
    /// Con `1` (el valor por defecto de HTML) elimina el atributo en vez de fijarlo.
    pub fn with_rowspan(mut self, span: u8) -> Self {
        self.props.alter_prop(if span == 1 {
            PropsOp::remove("rowspan")
        } else {
            PropsOp::set("rowspan", span.to_string())
        });
        self
    }

    /// Añade un nuevo componente a la celda o modifica la lista de componentes (`children`) con una
    /// operación [`ChildOp`].
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}

impl From<&str> for Cell {
    /// Convierte un `&str` en una celda de texto literal; equivale a `Cell::new(Lc::n(text))`.
    fn from(text: &str) -> Self {
        Cell::new(Lc::n(text.to_string()))
    }
}

impl From<String> for Cell {
    /// Convierte un `String` en una celda de texto literal; equivale a `Cell::new(Lc::n(text))`.
    fn from(text: String) -> Self {
        Cell::new(Lc::n(text))
    }
}

impl From<Lc> for Cell {
    /// Convierte un [`Lc`] en una celda traducible; equivale a `Cell::new(label)`.
    fn from(label: Lc) -> Self {
        Cell::new(label)
    }
}

impl<C: Component> From<C> for Cell {
    /// Convierte cualquier componente en una celda; equivale a `Cell::new(component)`.
    fn from(component: C) -> Self {
        Cell::new(component)
    }
}
