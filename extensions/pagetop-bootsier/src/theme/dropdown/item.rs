use pagetop::prelude::*;

// **< ItemType >***********************************************************************************

#[derive(AutoDefault)]
pub enum ItemType {
    #[default]
    Void,
    Label(L10n),
    Link(L10n, FnPathByContext),
    LinkBlank(L10n, FnPathByContext),
}

// **< Item >***************************************************************************************

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Item {
    item_type: ItemType,
}

impl Component for Item {
    fn new() -> Self {
        Item::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let description: Option<String> = None;

        // Obtiene la URL actual desde `cx.request`.
        let current_path = cx.request().map(|request| request.path());

        match self.item_type() {
            ItemType::Void => PrepareMarkup::None,
            ItemType::Label(label) => PrepareMarkup::With(html! {
                li class="dropdown-item" {
                    span title=[description] {
                        //(left_icon)
                        (label.using(cx))
                        //(right_icon)
                    }
                }
            }),
            ItemType::Link(label, path) => {
                let item_path = path(cx);
                let (class, aria) = if current_path == Some(item_path) {
                    ("dropdown-item active", Some("page"))
                } else {
                    ("dropdown-item", None)
                };
                PrepareMarkup::With(html! {
                    li class=(class) aria-current=[aria] {
                        a class="nav-link" href=(item_path) title=[description] {
                            //(left_icon)
                            (label.using(cx))
                            //(right_icon)
                        }
                    }
                })
            }
            ItemType::LinkBlank(label, path) => {
                let item_path = path(cx);
                let (class, aria) = if current_path == Some(item_path) {
                    ("dropdown-item active", Some("page"))
                } else {
                    ("dropdown-item", None)
                };
                PrepareMarkup::With(html! {
                    li class=(class) aria-current=[aria] {
                        a class="nav-link" href=(item_path) title=[description] target="_blank" {
                            //(left_icon)
                            (label.using(cx))
                            //(right_icon)
                        }
                    }
                })
            }
        }
    }
}

impl Item {
    pub fn label(label: L10n) -> Self {
        Item {
            item_type: ItemType::Label(label),
            ..Default::default()
        }
    }

    pub fn link(label: L10n, path: FnPathByContext) -> Self {
        Item {
            item_type: ItemType::Link(label, path),
            ..Default::default()
        }
    }

    pub fn link_blank(label: L10n, path: FnPathByContext) -> Self {
        Item {
            item_type: ItemType::LinkBlank(label, path),
            ..Default::default()
        }
    }

    // Item GETTERS.

    pub fn item_type(&self) -> &ItemType {
        &self.item_type
    }
}
