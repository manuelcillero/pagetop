use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Menu {
    id     : AttrId,
    classes: AttrClasses,
    items  : Children,
}

impl Component for Menu {
    fn new() -> Self {
        Menu::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, "menu");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        // cx.set_param::<bool>(PARAM_BASE_INCLUDE_MENU_ASSETS, &true);
        // cx.set_param::<bool>(PARAM_BASE_INCLUDE_ICONS, &true);

        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] {
                div class="menu__wrapper" {
                    div class="menu__panel" {
                        div class="menu__overlay" {}
                        nav class="menu__nav" {
                            div class="menu__header" {
                                button type="button" class="menu__back" {
                                    (Icon::svg(html! {
                                        path fill-rule="evenodd" d="M11.354 1.646a.5.5 0 0 1 0 .708L5.707 8l5.647 5.646a.5.5 0 0 1-.708.708l-6-6a.5.5 0 0 1 0-.708l6-6a.5.5 0 0 1 .708 0" {}
                                    }).render(cx))
                                }
                                div class="menu__title" {}
                                button type="button" class="menu__close" {
                                    (Icon::svg(html! {
                                        path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8z" {}
                                    }).render(cx))
                                }
                            }
                            ul class="menu__list" {
                                (self.items().render(cx))
                            }
                        }
                    }
                    button
                        type="button"
                        class="menu__trigger"
                        title=[L10n::l("menu_toggle").lookup(cx)]
                    {
                        (Icon::svg(html! {
                            path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5" {}
                        }).render(cx))
                    }
                }
            }
        })
    }
}

impl Menu {
    // **< Menu BUILDER >***************************************************************************

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

    /// Añade un nuevo ítem al menú.
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

    // **< Menu GETTERS >***************************************************************************

    /// Devuelve las clases CSS asociadas al menú.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve la lista de ítems (`children`) del menú.
    pub fn items(&self) -> &Children {
        &self.items
    }
}
