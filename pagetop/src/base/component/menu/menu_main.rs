use crate::prelude::*;
use crate::CrateHandle;

use super::Item;

#[rustfmt::skip]
#[derive(CrateHandle, SmartDefault)]
pub struct Menu {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    items     : TypedComponents<Item>,
}

impl ComponentTrait for Menu {
    fn new() -> Self {
        Menu::default()
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
}

impl Menu {
    // Menu BUILDER.

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

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

    #[rustfmt::skip]
    pub fn add_item(mut self, item: Item) -> Self {
        self.items.alter_value(ArcTypedOp::Add(ArcTypedComponent::new(item)));
        self
    }

    #[fn_builder]
    pub fn alter_items(&mut self, op: ArcTypedOp<Item>) -> &mut Self {
        self.items.alter_value(op);
        self
    }

    // Menu GETTERS.

    pub fn items(&self) -> &TypedComponents<Item> {
        &self.items
    }
}
