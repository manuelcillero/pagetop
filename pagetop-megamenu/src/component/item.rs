use pagetop::prelude::*;

use crate::component::MegaMenu;

new_handle!(COMPONENT_MEGAMENUITEM);

type Label = OneComponent<L10n>;
type Content = OneComponent<L10n>;

#[derive(Default)]
pub enum MegaMenuItemType {
    #[default]
    Void,
    Label(Label),
    Link(Label, String),
    LinkBlank(Label, String),
    Html(Content),
    Submenu(Label, MegaMenu),
    Separator,
}

// MegaMenuItem.

#[rustfmt::skip]
#[derive(Default)]
pub struct MegaMenuItem {
    weight    : Weight,
    renderable: Renderable,
    item_type : MegaMenuItemType,
}

impl ComponentTrait for MegaMenuItem {
    fn new() -> Self {
        MegaMenuItem::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_MEGAMENUITEM
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.item_type() {
            MegaMenuItemType::Void => PrepareMarkup::None,

            MegaMenuItemType::Label(label) => PrepareMarkup::With(html! {
                li class="label" { a href="#" { (label.prepare(cx)) } }
            }),
            MegaMenuItemType::Link(label, path) => PrepareMarkup::With(html! {
                li class="link" { a href=(path) { (label.prepare(cx)) } }
            }),
            MegaMenuItemType::LinkBlank(label, path) => PrepareMarkup::With(html! {
                li class="link_blank" {
                    a href=(path) target="_blank" { (label.prepare(cx)) }
                }
            }),
            MegaMenuItemType::Html(content) => PrepareMarkup::With(html! {
                li class="html" { (content.prepare(cx)) }
            }),
            MegaMenuItemType::Submenu(label, menu) => PrepareMarkup::With(html! {
                li class="submenu" {
                    a href="#" { (label.prepare(cx)) }
                    ul {
                        (menu.items().prepare(cx))
                    }
                }
            }),
            MegaMenuItemType::Separator => PrepareMarkup::With(html! {
                li class="separator" { }
            }),
        }
    }
}

impl MegaMenuItem {
    pub fn label(label: L10n) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Label(OneComponent::new_with(label)),
            ..Default::default()
        }
    }

    pub fn link(label: L10n, path: &str) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Link(OneComponent::new_with(label), path.to_owned()),
            ..Default::default()
        }
    }

    pub fn link_blank(label: L10n, path: &str) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::LinkBlank(OneComponent::new_with(label), path.to_owned()),
            ..Default::default()
        }
    }

    pub fn html(content: L10n) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Html(OneComponent::new_with(content)),
            ..Default::default()
        }
    }

    pub fn submenu(label: L10n, menu: MegaMenu) -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Submenu(OneComponent::new_with(label), menu),
            ..Default::default()
        }
    }

    pub fn separator() -> Self {
        MegaMenuItem {
            item_type: MegaMenuItemType::Separator,
            ..Default::default()
        }
    }

    // MegaMenuItem BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    // MegaMenuItem GETTERS.

    pub fn item_type(&self) -> &MegaMenuItemType {
        &self.item_type
    }
}
