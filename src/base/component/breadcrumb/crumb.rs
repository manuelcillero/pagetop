use crate::prelude::*;

/// Representa un elemento de un [`Breadcrumb`](super::Breadcrumb).
///
/// Hay tres formas de crear un `Crumb`, según el papel que ocupe en la lista:
///
/// - [`Crumb::new()`]: un elemento enlazado, con destino propio.
/// - [`Crumb::current()`]: el elemento final, sin enlace, que representa la página actual.
/// - [`Crumb::text()`]: texto plano sin enlace y sin marcar como página actual, por ejemplo un
///   nivel intermedio sin URL propia.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let bc = Breadcrumb::new()
///     .with_crumb(breadcrumb::Crumb::new(L10n::n("Home"), "/"))
///     .with_crumb(breadcrumb::Crumb::text(L10n::n("Users")))
///     .with_crumb(breadcrumb::Crumb::current(L10n::n("Julia")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Crumb {
    /// Devuelve identificador, clases CSS y atributos HTML del elemento.
    props: Props,
    /// Devuelve la etiqueta del elemento.
    label: L10n,
    /// Devuelve la ruta de destino del elemento, si es un enlace.
    route: Option<Route>,
    /// Devuelve si el elemento representa la página actual.
    is_current: bool,
}

impl Crumb {
    /// Crea un elemento enlazado a la ruta indicada.
    pub fn new(label: L10n, route: impl Into<Route>) -> Self {
        Self {
            label,
            route: Some(route.into()),
            is_current: false,
            ..Default::default()
        }
    }

    /// Crea el elemento final de la lista, sin enlace, que representa la página actual.
    ///
    /// Al renderizarse dentro de un `Breadcrumb`, el elemento lleva `aria-current="page"` y la
    /// clase `.active`.
    pub fn current(label: L10n) -> Self {
        Self {
            label,
            is_current: true,
            ..Default::default()
        }
    }

    /// Crea un elemento de sólo texto, sin enlace ni marca de página actual (por ejemplo, un nivel
    /// intermedio sin URL propia).
    pub fn text(label: L10n) -> Self {
        Self {
            label,
            ..Default::default()
        }
    }

    // **< Crumb BUILDER >**************************************************************************

    /// Establece el identificador único del elemento.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS o atributos HTML del elemento.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    // Normaliza la clase base según el papel del elemento. Sólo lo usa `Breadcrumb`.
    pub(super) fn setup(&mut self, _cx: &Context) {
        if *self.is_current() {
            self.alter_prop(PropsOp::prepend_classes("active"))
                .alter_prop(PropsOp::set("aria-current", "page"));
        }
        self.alter_prop(PropsOp::prepend_classes("breadcrumb-item"));
    }

    // Renderiza con enlace si tiene ruta, o texto plano en otro caso. Sólo lo usa `Breadcrumb`.
    pub(super) fn render_crumb(&self, cx: &Context) -> Markup {
        let label = self.label().using(cx);
        match self.route() {
            Some(route) => html! {
                li (self.props()) {
                    a href=(route.resolve(cx).to_string()) { (label) }
                }
            },
            None => html! {
                li (self.props()) { (label) }
            },
        }
    }
}
