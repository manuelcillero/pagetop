use crate::prelude::*;
use crate::CrateHandle;

use super::{Megamenu, Submenu};

type Label = L10n;
type Content = ArcTypedComponent<Html>;
type SubmenuItems = ArcTypedComponent<Submenu>;
type MegamenuGroups = ArcTypedComponent<Megamenu>;

#[derive(SmartDefault)]
pub enum ItemType {
    #[default]
    Void,
    Label(Label),
    Link(Label, FnContextualPath),
    LinkBlank(Label, FnContextualPath),
    Html(Content),
    Submenu(Label, SubmenuItems),
    Megamenu(Label, MegamenuGroups),
}

// Item.

#[rustfmt::skip]
#[derive(CrateHandle, SmartDefault)]
pub struct Item {
    weight     : Weight,
    renderable : Renderable,
    item_type  : ItemType,
    description: OptionTranslated,
    left_icon  : OptionComponent<Icon>,
    right_icon : OptionComponent<Icon>,
}

impl ComponentTrait for Item {
    fn new() -> Self {
        Item::default()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let description = self.description.using(cx.langid());

        let left_icon = self.left_icon().render(cx);
        let right_icon = self.right_icon().render(cx);

        match self.item_type() {
            ItemType::Void => PrepareMarkup::None,
            ItemType::Label(label) => PrepareMarkup::With(html! {
                li class="pt-menu__label" {
                    span title=[description] {
                        (left_icon)
                        (label.escaped(cx.langid()))
                        (right_icon)
                    }
                }
            }),
            ItemType::Link(label, path) => PrepareMarkup::With(html! {
                li class="pt-menu__link" {
                    a href=(path(cx)) title=[description] {
                        (left_icon)
                        (label.escaped(cx.langid()))
                        (right_icon)
                    }
                }
            }),
            ItemType::LinkBlank(label, path) => PrepareMarkup::With(html! {
                li class="pt-menu__link" {
                    a href=(path(cx)) title=[description] target="_blank" {
                        (left_icon)
                        (label.escaped(cx.langid()))
                        (right_icon)
                    }
                }
            }),
            ItemType::Html(content) => PrepareMarkup::With(html! {
                li class="pt-menu__html" {
                    (content.render(cx))
                }
            }),
            ItemType::Submenu(label, submenu) => PrepareMarkup::With(html! {
                li class="pt-menu__children" {
                    a href="#" title=[description] {
                        (left_icon)
                        (label.escaped(cx.langid())) i class="pt-menu__icon bi-chevron-down" {}
                    }
                    div class="pt-menu__subs" {
                        (submenu.render(cx))
                    }
                }
            }),
            ItemType::Megamenu(label, megamenu) => PrepareMarkup::With(html! {
                li class="pt-menu__children" {
                    a href="#" title=[description] {
                        (left_icon)
                        (label.escaped(cx.langid())) i class="pt-menu__icon bi-chevron-down" {}
                    }
                    div class="pt-menu__subs pt-menu__mega" {
                        (megamenu.render(cx))
                    }
                }
            }),
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

    pub fn link(label: L10n, path: FnContextualPath) -> Self {
        Item {
            item_type: ItemType::Link(label, path),
            ..Default::default()
        }
    }

    pub fn link_blank(label: L10n, path: FnContextualPath) -> Self {
        Item {
            item_type: ItemType::LinkBlank(label, path),
            ..Default::default()
        }
    }

    pub fn html(content: Html) -> Self {
        Item {
            item_type: ItemType::Html(Content::new(content)),
            ..Default::default()
        }
    }

    pub fn submenu(label: L10n, submenu: Submenu) -> Self {
        Item {
            item_type: ItemType::Submenu(label, SubmenuItems::new(submenu)),
            ..Default::default()
        }
    }

    pub fn megamenu(label: L10n, megamenu: Megamenu) -> Self {
        Item {
            item_type: ItemType::Megamenu(label, MegamenuGroups::new(megamenu)),
            ..Default::default()
        }
    }

    // Item BUILDER.

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
    pub fn alter_description(&mut self, text: L10n) -> &mut Self {
        self.description.alter_value(text);
        self
    }

    #[fn_builder]
    pub fn alter_left_icon(&mut self, icon: Option<Icon>) -> &mut Self {
        self.left_icon.alter_value(icon);
        self
    }

    #[fn_builder]
    pub fn alter_right_icon(&mut self, icon: Option<Icon>) -> &mut Self {
        self.right_icon.alter_value(icon);
        self
    }

    // Item GETTERS.

    pub fn item_type(&self) -> &ItemType {
        &self.item_type
    }

    pub fn description(&self) -> &OptionTranslated {
        &self.description
    }

    pub fn left_icon(&self) -> &OptionComponent<Icon> {
        &self.left_icon
    }

    pub fn right_icon(&self) -> &OptionComponent<Icon> {
        &self.right_icon
    }
}
