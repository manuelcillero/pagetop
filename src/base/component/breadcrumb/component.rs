use crate::prelude::*;

/// Componente para representar una **ruta de navegación** (*breadcrumb*).
///
/// Renderiza la estructura HTML de cualquier breadcrumb encapsulando en un elemento `<nav>` una
/// lista con un [`breadcrumb::Crumb`] por cada nivel de la ruta de navegación. Deja en manos de
/// quien lo use la construcción de esos niveles. No sabe nada del origen de los datos, ni de menús,
/// ni de ningún otro esquema de navegación concreto; cada extensión resuelve sus propios datos (por
/// ejemplo, el *active trail* de un menú) y construye los [`breadcrumb::Crumb`] correspondientes.
///
/// Si no contiene ningún elemento, el componente **no se renderiza**.
///
/// # Clases CSS
///
/// - `.breadcrumb`: clase de la lista que contiene los niveles.
/// - `.breadcrumb-item`: presente en todos los elementos de la lista.
/// - `.active`: añadida al elemento actual (ver [`breadcrumb::Crumb::current()`]).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let bc = Breadcrumb::new()
///     .with_crumb(breadcrumb::Crumb::new(Lc::n("Home"), "/"))
///     .with_crumb(breadcrumb::Crumb::new(Lc::n("Users"), "/admin/users"))
///     .with_crumb(breadcrumb::Crumb::current(Lc::n("Julia")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Breadcrumb {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la lista de elementos del breadcrumb, en orden de aparición.
    crumbs: Vec<breadcrumb::Crumb>,
}

#[async_trait]
impl Component for Breadcrumb {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &Context) {
        for crumb in self.crumbs.iter_mut() {
            crumb.setup(cx);
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        if self.crumbs().is_empty() {
            return Ok(html! {});
        }

        Ok(html! {
            nav (self.props().unpack(cx)) aria-label=[Lc::l("breadcrumb_label").lookup(cx)] {
                ol.breadcrumb {
                    @for crumb in self.crumbs() {
                        (crumb.render_crumb(cx))
                    }
                }
            }
        })
    }
}

#[builder_impl]
impl Breadcrumb {
    // **< Breadcrumb BUILDER >*********************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Añade un nuevo elemento al final del breadcrumb.
    pub fn with_crumb(mut self, crumb: breadcrumb::Crumb) -> Self {
        self.crumbs.push(crumb);
        self
    }
}
