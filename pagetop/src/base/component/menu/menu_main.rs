use crate::prelude::*;

use super::Item;

new_handle!(COMPONENT_BASE_MENU);

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
        COMPONENT_BASE_MENU
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
        cx.set_param::<bool>(PARAM_BASE_INCLUDE_MENU_ASSETS, true);
        cx.set_param::<bool>(PARAM_BASE_INCLUDE_ICONS, true);

        PrepareMarkup::With(html! {
            div id=[self.id()] class="pt-menu__container" {
                div class="pt-menu__wrapper" {
                    div class="pt-menu__main" {
                        div class="pt-menu__overlay" {}
                        nav class="pt-menu__nav" {
                            div class="pt-menu__header" {
                                button type="button" class="pt-menu__arrow" {
                                    i class="bi-chevron-left" {}
                                }
                                div class="pt-menu__title" {}
                                button type="button" class="pt-menu__close" {
                                    i class="bi-x" {}
                                }
                            }
                            ul class="pt-menu__section" {
                                (self.items().render(cx))
                            }
                        }
                    }
                    button
                        type="button"
                        class="pt-menu__trigger"
                        title=[L10n::l("menu_toggle").using(cx.langid())]
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

    pub fn add_item(mut self, item: Item) -> Self {
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
