use pagetop::prelude::*;

use crate::theme::*;

/// Componente para crear un **menú** ([`nav`](crate::theme::bs::nav)).
///
/// Presenta un menú con una lista de elementos usando una vista básica, o alguna de sus variantes
/// ([`nav::Kind`](crate::theme::bs::nav::Kind)) como *pestañas* (`Tabs`), *botones* (`Pills`) o
/// *subrayado* (`Underline`).
/// También permite controlar su distribución y orientación
/// ([`nav::Layout`](crate::theme::bs::nav::Layout)).
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let nav = bs::Nav::tabs()
///     .with_layout(bs::nav::Layout::End)
///     .with_item(bs::nav::Item::link(L10n::n("Home"), |_| "/".into()))
///     .with_item(bs::nav::Item::link_blank(L10n::n("External"), |_| "https://docs.rs".into()))
///     .with_item(bs::nav::Item::dropdown(
///         bs::Dropdown::new()
///             .with_title(L10n::n("Options"))
///             .with_item(ChildOp::AddMany(vec![
///                 bs::dropdown::Item::link(L10n::n("Action"), |_| "/action".into()).into(),
///                 bs::dropdown::Item::link(L10n::n("Another"), |_| "/another".into()).into(),
///             ])),
///     ))
///     .with_item(bs::nav::Item::link_disabled(L10n::n("Disabled"), |_| "#".into()));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Nav {
    /// Devuelve identificador, clases CSS y atributos HTML del componente.
    props: Props,
    /// Devuelve el estilo visual seleccionado.
    nav_kind: bs::nav::Kind,
    /// Devuelve la distribución y orientación seleccionada.
    nav_layout: bs::nav::Layout,
    /// Devuelve la lista de elementos del menú.
    items: Children,
}

impl Component for Nav {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        // Clases CSS por defecto para el menú, según el estilo y la distribución seleccionados.
        self.alter_prop(PropsOp::prepend_classes({
            let mut classes = "nav".to_string();
            self.nav_kind().push_to(&mut classes);
            self.nav_layout().push_to(&mut classes);
            classes
        }));
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let items = self.items().render(cx);
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
    /// Crea un `Nav` usando pestañas para los elementos (*Tabs*).
    pub fn tabs() -> Self {
        Self::default().with_kind(bs::nav::Kind::Tabs)
    }

    /// Crea un `Nav` usando botones para los elementos (*Pills*).
    pub fn pills() -> Self {
        Self::default().with_kind(bs::nav::Kind::Pills)
    }

    /// Crea un `Nav` usando elementos subrayados (*Underline*).
    pub fn underline() -> Self {
        Self::default().with_kind(bs::nav::Kind::Underline)
    }

    // **< Nav BUILDER >****************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS o atributos HTML del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Cambia el estilo del menú (*Tabs*, *Pills*, *Underline* o *Default*).
    #[builder_fn]
    pub fn with_kind(mut self, kind: bs::nav::Kind) -> Self {
        self.nav_kind = kind;
        self
    }

    /// Selecciona la distribución y orientación del menú.
    #[builder_fn]
    pub fn with_layout(mut self, layout: bs::nav::Layout) -> Self {
        self.nav_layout = layout;
        self
    }

    /// Añade un nuevo elemento al menú o modifica la lista de elementos del menú con una operación
    /// [`ChildOp`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// nav.with_item(nav::Item::link("Inicio", "/"));
    /// nav.with_item(ChildOp::AddMany(vec![
    ///     nav::Item::link(...).into(),
    ///     nav::Item::link_disabled(...).into(),
    /// ]));
    /// ```
    #[builder_fn]
    pub fn with_item(mut self, op: impl Into<ChildOp>) -> Self {
        self.items.alter_child(op.into());
        self
    }
}
