use crate::prelude::*;

//use super::{Megamenu, Submenu};

type Label = L10n;
type Content = Typed<Html>;
type SubmenuItems = Typed<menu::Submenu>;
//type MegamenuGroups = Typed<Megamenu>;

#[derive(AutoDefault)]
pub enum ItemKind {
    #[default]
    Void,
    Label(Label),
    Link(Label, FnPathByContext),
    LinkBlank(Label, FnPathByContext),
    Html(Content),
    Submenu(Label, SubmenuItems),
    // Megamenu(Label, MegamenuGroups),
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Item {
    item_kind  : ItemKind,
    description: AttrL10n,
    left_icon  : Typed<Icon>,
    right_icon : Typed<Icon>,
}

impl Component for Item {
    fn new() -> Self {
        Item::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let description = self.description().lookup(cx);
        let left_icon = self.left_icon().render(cx);
        let right_icon = self.right_icon().render(cx);

        match self.item_kind() {
            ItemKind::Void => PrepareMarkup::None,
            ItemKind::Label(label) => PrepareMarkup::With(html! {
                li class="menu__label" {
                    span title=[description] {
                        (left_icon)
                        (label.using(cx))
                        (right_icon)
                    }
                }
            }),
            ItemKind::Link(label, path) => PrepareMarkup::With(html! {
                li class="menu__link" {
                    a href=(path(cx)) title=[description] {
                        (left_icon)
                        (label.using(cx))
                        (right_icon)
                    }
                }
            }),
            ItemKind::LinkBlank(label, path) => PrepareMarkup::With(html! {
                li class="menu__link" {
                    a href=(path(cx)) title=[description] target="_blank" {
                        (left_icon)
                        (label.using(cx))
                        (right_icon)
                    }
                }
            }),
            ItemKind::Html(content) => PrepareMarkup::With(html! {
                li class="menu__html" {
                    (content.render(cx))
                }
            }),
            ItemKind::Submenu(label, submenu) => PrepareMarkup::With(html! {
                li class="menu__children" {
                    a href="#" title=[description] {
                        (left_icon)
                        (label.using(cx)) i class="menu__icon bi-chevron-down" {}
                    }
                    div class="menu__subs" {
                        (submenu.render(cx))
                    }
                }
            }),
            /*
            ItemKind::Megamenu(label, megamenu) => PrepareMarkup::With(html! {
                li class="menu__children" {
                    a href="#" title=[description] {
                        (left_icon)
                        (label.escaped(cx.langid())) i class="menu__icon bi-chevron-down" {}
                    }
                    div class="menu__subs menu__mega" {
                        (megamenu.render(cx))
                    }
                }
            }),
            */
        }
    }
}

impl Item {
    pub fn label(label: L10n) -> Self {
        Item {
            item_kind: ItemKind::Label(label),
            ..Default::default()
        }
    }

    pub fn link(label: L10n, path: FnPathByContext) -> Self {
        Item {
            item_kind: ItemKind::Link(label, path),
            ..Default::default()
        }
    }

    pub fn link_blank(label: L10n, path: FnPathByContext) -> Self {
        Item {
            item_kind: ItemKind::LinkBlank(label, path),
            ..Default::default()
        }
    }

    pub fn html(content: Html) -> Self {
        Item {
            item_kind: ItemKind::Html(Content::with(content)),
            ..Default::default()
        }
    }

    pub fn submenu(label: L10n, submenu: menu::Submenu) -> Self {
        Item {
            item_kind: ItemKind::Submenu(label, SubmenuItems::with(submenu)),
            ..Default::default()
        }
    }
    /*
    pub fn megamenu(label: L10n, megamenu: Megamenu) -> Self {
        Item {
            item_kind: ItemKind::Megamenu(label, MegamenuGroups::with(megamenu)),
            ..Default::default()
        }
    }
    */
    // **< Item BUILDER >***************************************************************************

    #[builder_fn]
    pub fn with_description(mut self, text: L10n) -> Self {
        self.description.alter_value(text);
        self
    }

    #[builder_fn]
    pub fn with_left_icon<I: Into<Icon>>(mut self, icon: Option<I>) -> Self {
        self.left_icon.alter_component(icon.map(Into::into));
        self
    }

    #[builder_fn]
    pub fn with_right_icon<I: Into<Icon>>(mut self, icon: Option<I>) -> Self {
        self.right_icon.alter_component(icon.map(Into::into));
        self
    }

    // **< Item GETTERS >***************************************************************************

    pub fn item_kind(&self) -> &ItemKind {
        &self.item_kind
    }

    pub fn description(&self) -> &AttrL10n {
        &self.description
    }

    pub fn left_icon(&self) -> &Typed<Icon> {
        &self.left_icon
    }

    pub fn right_icon(&self) -> &Typed<Icon> {
        &self.right_icon
    }
}
