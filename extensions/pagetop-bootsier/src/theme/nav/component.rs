use pagetop::prelude::*;

use crate::prelude::*;

/// Componente para crear un **menú** o alguna de sus variantes ([`nav::Kind`]).
///
/// Presenta un menú con una lista de elementos usando una vista básica, o alguna de sus variantes
/// como *pestañas* (`Tabs`), *botones* (`Pills`) o *subrayado* (`Underline`). También permite
/// controlar su distribución y orientación ([`nav::Layout`](crate::theme::nav::Layout)).
///
/// Ver ejemplo en el módulo [`nav`].
/// Si no contiene elementos, el componente **no se renderiza**.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Nav {
    id        : AttrId,
    classes   : AttrClasses,
    items     : Children,
    nav_kind  : nav::Kind,
    nav_layout: nav::Layout,
}

impl Component for Nav {
    fn new() -> Self {
        Nav::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                "nav",
                match self.nav_kind() {
                    nav::Kind::Default => "",
                    nav::Kind::Tabs => "nav-tabs",
                    nav::Kind::Pills => "nav-pills",
                    nav::Kind::Underline => "nav-underline",
                },
                match self.nav_layout() {
                    nav::Layout::Default => "",
                    nav::Layout::Start => "justify-content-start",
                    nav::Layout::Center => "justify-content-center",
                    nav::Layout::End => "justify-content-end",
                    nav::Layout::Vertical => "flex-column",
                    nav::Layout::Fill => "nav-fill",
                    nav::Layout::Justified => "nav-justified",
                },
            ]
            .join_classes(),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let items = self.items().render(cx);
        if items.is_empty() {
            return PrepareMarkup::None;
        }

        PrepareMarkup::With(html! {
            ul id=[self.id()] class=[self.classes().get()] {
                (items)
            }
        })
    }
}

impl Nav {
    /// Crea un `Nav` usando pestañas para los elementos (*Tabs*).
    pub fn tabs() -> Self {
        Nav::default().with_kind(nav::Kind::Tabs)
    }

    /// Crea un `Nav` usando botones para los elementos (*Pills*).
    pub fn pills() -> Self {
        Nav::default().with_kind(nav::Kind::Pills)
    }

    /// Crea un `Nav` usando elementos subrayados (*Underline*).
    pub fn underline() -> Self {
        Nav::default().with_kind(nav::Kind::Underline)
    }

    // **< Nav BUILDER >****************************************************************************

    /// Establece el identificador único (`id`) del menú.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al menú.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    /// Cambia el estilo del menú (*Tabs*, *Pills*, *Underline* o *Default*).
    #[builder_fn]
    pub fn with_kind(mut self, kind: nav::Kind) -> Self {
        self.nav_kind = kind;
        self
    }

    /// Selecciona la distribución y orientación del menú.
    #[builder_fn]
    pub fn with_layout(mut self, layout: nav::Layout) -> Self {
        self.nav_layout = layout;
        self
    }

    /// Añade un nuevo elemento hijo al menú.
    pub fn add_item(mut self, item: nav::Item) -> Self {
        self.items.add(Child::with(item));
        self
    }

    /// Modifica la lista de elementos (`children`) aplicando una operación [`TypedOp`].
    #[builder_fn]
    pub fn with_items(mut self, op: TypedOp<nav::Item>) -> Self {
        self.items.alter_typed(op);
        self
    }

    // **< Nav GETTERS >****************************************************************************

    /// Devuelve las clases CSS asociadas al menú.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve el estilo visual seleccionado.
    pub fn nav_kind(&self) -> &nav::Kind {
        &self.nav_kind
    }

    /// Devuelve la distribución y orientación seleccionada.
    pub fn nav_layout(&self) -> &nav::Layout {
        &self.nav_layout
    }

    /// Devuelve la lista de elementos (`children`) del menú.
    pub fn items(&self) -> &Children {
        &self.items
    }
}
