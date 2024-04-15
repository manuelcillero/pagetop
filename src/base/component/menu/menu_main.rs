use crate::prelude::*;

use super::Item;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Menu {
    id   : OptionId,
    items: MixedComponents,
}

impl ComponentTrait for Menu {
    fn new() -> Self {
        Menu::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        cx.set_param::<bool>(PARAM_BASE_INCLUDE_MENU_ASSETS, true);
        cx.set_param::<bool>(PARAM_BASE_INCLUDE_ICONS, true);

        PrepareMarkup::With(html! {
            div id=[self.id()] class="menu__container" {
                div class="menu__content" {
                    div class="menu__main" {
                        div class="menu__overlay" {}
                        nav class="menu__nav" {
                            div class="menu__header" {
                                button type="button" class="menu__arrow" {
                                    i class="bi-chevron-left" {}
                                }
                                div class="menu__title" {}
                                button type="button" class="menu__close" {
                                    i class="bi-x" {}
                                }
                            }
                            ul class="menu__section" {
                                (self.items().render(cx))
                            }
                        }
                    }
                    button
                        type="button"
                        class="menu__trigger"
                        title=[L10n::l("menu_toggle").using(cx.langid())]
                    {
                        span {} span {} span {}
                    }
                }
            }
        })
    }
}

impl Menu {
    // Menu BUILDER.

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
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

    // Menu GETTERS.

    pub fn items(&self) -> &MixedComponents {
        &self.items
    }
}
