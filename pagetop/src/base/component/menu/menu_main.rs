use crate::prelude::*;
use crate::LOCALES_PAGETOP;

use super::Item;

new_handle!(COMPONENT_MENU);

actions_for_component!(Menu);

#[rustfmt::skip]
#[derive(Default)]
pub struct Menu {
    weight    : Weight,
    renderable: Renderable,
    id        : OptionId,
    items     : TypedComponents<Item>,
}

impl ComponentTrait for Menu {
    fn new() -> Self {
        Menu::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_MENU
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

    fn before_prepare_component(&mut self, cx: &mut Context) {
        run_actions_before_prepare_menu(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        cx.set_param::<bool>(PARAM_INCLUDE_MENU_ASSETS, true);
        cx.set_param::<bool>(PARAM_INCLUDE_ICONS, true);

        PrepareMarkup::With(html! {
            div id=[self.id()] class="menu-container" {
                div class="menu-wrapper" {
                    div class="menu-main" {
                        div class="menu-overlay" {}
                        nav class="menu" {
                            div class="menu-header" {
                                button type="button" class="menu-arrow" {
                                    i class="bi-chevron-left" {}
                                }
                                div class="menu-title" {}
                                button type="button" class="menu-close" {
                                    i class="bi-x" {}
                                }
                            }
                            ul class="menu-section" {
                                (self.items().prepare(cx))
                            }
                        }
                    }
                    button
                        type="button"
                        class="menu-trigger"
                        title=[L10n::t("menu_toggle", &LOCALES_PAGETOP).into_string(cx)]
                    {
                        span {} span {} span {}
                    }
                }
            }
        })
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        run_actions_after_prepare_menu(self, cx);
    }
}

impl Menu {
    // Menu BUILDER.

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

    pub fn with_item(mut self, item: Item) -> Self {
        self.items.alter(TypedOp::Add(TypedComponent::with(item)));
        self
    }

    #[fn_builder]
    pub fn alter_items(&mut self, op: TypedOp<Item>) -> &mut Self {
        self.items.alter(op);
        self
    }

    // Menu GETTERS.

    pub fn items(&self) -> &TypedComponents<Item> {
        &self.items
    }
}
