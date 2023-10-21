use crate::prelude::*;

use super::Item;

new_handle!(COMPONENT_BASE_MENU_SUBMENU);

type Items = TypedComponents<Item>;

#[rustfmt::skip]
#[derive(Default)]
pub struct Submenu {
    weight    : Weight,
    renderable: Renderable,
    id        : OptionId,
    title     : OptionTranslate,
    items     : Items,
}

impl ComponentTrait for Submenu {
    fn new() -> Self {
        Submenu::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_MENU_SUBMENU
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="pt-menu__items" {
                @if let Some(title) = self.title().using(cx.langid()) {
                    h4 class="pt-menu__title" { (title) }
                }
                ul {
                    (self.items().prepare(cx))
                }
            }
        })
    }
}

impl Submenu {
    // Submenu BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_title(&mut self, title: L10n) -> &mut Self {
        self.title.alter_value(title);
        self
    }

    pub fn add_item(mut self, item: Item) -> Self {
        self.items.alter(TypedOp::Add(TypedComponent::with(item)));
        self
    }

    #[fn_builder]
    pub fn alter_items(&mut self, op: TypedOp<Item>) -> &mut Self {
        self.items.alter(op);
        self
    }

    // Submenu GETTERS.

    pub fn title(&self) -> &OptionTranslate {
        &self.title
    }

    pub fn items(&self) -> &Items {
        &self.items
    }
}
