use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Submenu {
    id   : AttrId,
    title: AttrL10n,
    items: Children,
}

impl Component for Submenu {
    fn new() -> Self {
        Submenu::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="menu__submenu" {
                @if let Some(title) = self.title().lookup(cx) {
                    h4 class="menu__submenu-title" { (title) }
                }
                ul {
                    (self.items().render(cx))
                }
            }
        })
    }
}

impl Submenu {
    // **< Submenu BUILDER >************************************************************************

    /// Establece el identificador único (`id`) del submenú.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title.alter_value(title);
        self
    }

    /// Añade un nuevo ítem al submenú.
    pub fn add_item(mut self, item: menu::Item) -> Self {
        self.items.alter_typed(TypedOp::Add(Typed::with(item)));
        self
    }

    /// Modifica la lista de ítems (`children`) aplicando una operación [`TypedOp`].
    #[builder_fn]
    pub fn with_items(mut self, op: TypedOp<menu::Item>) -> Self {
        self.items.alter_typed(op);
        self
    }

    // **< Submenu GETTERS >************************************************************************

    pub fn title(&self) -> &AttrL10n {
        &self.title
    }

    /// Devuelve la lista de ítems (`children`) del submenú.
    pub fn items(&self) -> &Children {
        &self.items
    }
}
