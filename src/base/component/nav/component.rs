use crate::prelude::*;

/// Componente para crear un **menú de navegación plano**.
///
/// Presenta una lista de elementos [`nav::Item`](super::Item), donde alguno puede desplegar un
/// [`Dropdown`](super::super::Dropdown) embebido.
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let nav = nav::Nav::new()
///     .with_item(nav::Item::link(Lc::n("Home"), "/"))
///     .with_item(nav::Item::link_blank(Lc::n("External"), "https://docs.rs"))
///     .with_item(nav::Item::dropdown(
///         dropdown::Dropdown::new()
///             .with_title(Lc::n("Options"))
///             .with_item(dropdown::Item::link(Lc::n("Action"), "/action"))
///             .with_item(dropdown::Item::link(Lc::n("Another"), "/another")),
///     ))
///     .with_item(nav::Item::link_disabled(Lc::n("Disabled"), "#"));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Nav {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la distribución y orientación seleccionada.
    nav_layout: nav::Layout,
    /// Devuelve la lista de elementos del menú.
    items: Children,
}

#[async_trait]
impl Component for Nav {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes(match self.nav_layout() {
            nav::Layout::Default => "nav",
            nav::Layout::Start => "nav nav-start",
            nav::Layout::Center => "nav nav-center",
            nav::Layout::End => "nav nav-end",
            nav::Layout::Vertical => "nav nav-vertical",
            nav::Layout::Fill => "nav nav-fill",
            nav::Layout::Justified => "nav nav-justified",
        }));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let items = self.items().render(cx).await;
        if items.is_empty() {
            return Ok(html! {});
        }

        Ok(html! {
            ul (self.props()) {
                (items)
            }
        })
    }
}

impl Nav {
    // **< Nav BUILDER >****************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Selecciona la distribución y orientación del menú.
    #[builder_fn]
    pub fn with_layout(mut self, layout: nav::Layout) -> Self {
        self.nav_layout = layout;
        self
    }

    /// Añade un nuevo elemento al menú o modifica la lista de elementos del menú con una
    /// operación [`TypedOp`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// nav.with_item(nav::Item::link("Inicio", "/"));
    /// nav.with_item(TypedOp::AddMany(vec![
    ///     nav::Item::link(...),
    ///     nav::Item::link_disabled(...),
    /// ]));
    /// ```
    #[builder_fn]
    pub fn with_item(mut self, op: impl Into<TypedOp<nav::Item>>) -> Self {
        self.items.alter_child(op.into());
        self
    }
}
