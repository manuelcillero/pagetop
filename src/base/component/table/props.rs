use crate::prelude::*;

/// Enlace de ordenación para una cabecera de columna ([`Column::with_sort`]).
///
/// Encapsula la URL de destino (`href`) y, si la tabla está actualmente ordenada por esta columna,
/// la dirección vigente (`dir`). [`Table`] usa esa información para marcar la cabecera con
/// `aria-sort` y aplicar las clases `table-sort`/`table-sort-asc`/`table-sort-desc` al enlace, de
/// modo que el tema activo pueda mostrar el indicador visual con CSS.
///
/// `SortLink` no añade ningún atributo adicional por sí sola: el enlace ya es funcional por sí
/// mismo (navega a `href` con una petición normal). Se pueden añadir atributos adicionales para
/// opciones de interactividad usando [`with_prop()`].
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let link = table::SortLink::new("/admin/users?sort=email")
///     .with_id("sort-email")
///     .with_dir(SortDir::Desc)
///     .with_prop(PropsOp::set("data-sort", "email"));
///
/// assert_eq!(link.props().get_id(), Some("sort-email".to_string()));
/// assert_eq!(link.href().as_deref(), Some("/admin/users?sort=email"));
/// assert_eq!(link.dir(), Some(&SortDir::Desc));
/// ```
///
/// [`Table`]: super::Table
/// [`Column::with_sort`]: super::Column::with_sort
/// [`with_prop()`]: Self::with_prop
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct SortLink {
    /// Devuelve los atributos adicionales del enlace.
    props: Props,
    /// Devuelve la URL de destino del enlace ya normalizada.
    href: AttrValue,
    /// Devuelve la dirección de orden vigente si la tabla está ordenada por esta columna, o
    /// `None` si la columna es ordenable pero no es la que determina el orden actual.
    dir: Option<SortDir>,
}

impl SortLink {
    /// Crea un enlace de ordenación hacia la URL indicada, sin dirección activa.
    pub fn new(href: impl Into<RoutePath>) -> Self {
        Self {
            href: AttrValue::new(href.into().to_string()),
            ..Default::default()
        }
    }

    // **< SortLink BUILDER >***********************************************************************

    /// Establece el identificador único del enlace.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Establece la dirección de orden vigente, o `None` si esta columna no es la que ordena
    /// actualmente la tabla.
    #[builder_fn]
    pub fn with_dir(mut self, dir: impl Into<Option<SortDir>>) -> Self {
        self.dir = dir.into();
        self
    }

    /// Modifica los atributos HTML del enlace. Es el punto de extensión para añadir atributos de
    /// interactividad sin que `Table` necesite conocerlos.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    // Determina el `aria-sort` y los atributos del enlace ya combinados con las clases
    // `table-sort`/`table-sort-asc`/`table-sort-desc` correspondientes al estado de orden vigente.
    // Sólo lo usa `Column` para montar la cabecera completa; no forma parte de la API pública.
    pub(super) fn header_attrs(&self) -> (&'static str, Props) {
        let (aria_sort, sort_class) = match self.dir() {
            Some(dir) => match dir {
                SortDir::Asc => ("ascending", "table-sort table-sort-asc"),
                SortDir::Desc => ("descending", "table-sort table-sort-desc"),
            },
            None => ("none", "table-sort"),
        };
        let props = self
            .props()
            .clone()
            .with_prop(PropsOp::prepend_classes(sort_class));
        (aria_sort, props)
    }
}
