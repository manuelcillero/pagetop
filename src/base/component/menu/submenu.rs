use crate::prelude::*;

use super::Item;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Submenu {
    id   : OptionId,
    title: OptionTranslated,
    items: MixedComponents,
}

impl ComponentTrait for Submenu {
    fn new() -> Self {
        Submenu::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="menu__items" {
                @if let Some(title) = self.title().using(cx.langid()) {
                    h4 class="menu__title" { (title) }
                }
                ul {
                    (self.items().render(cx))
                }
            }
        })
    }
}

impl Submenu {
    // Submenu BUILDER.

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

    #[fn_builder]
    pub fn alter_items(&mut self, op: TypedOp<Item>) -> &mut Self {
        self.items.alter_typed(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_item(mut self, item: Item) -> Self {
        self.items.alter_value(AnyOp::Add(AnyComponent::with(item)));
        self
    }

    // Submenu GETTERS.

    pub fn title(&self) -> &OptionTranslated {
        &self.title
    }

    pub fn items(&self) -> &MixedComponents {
        &self.items
    }
}
