use pagetop::prelude::*;

use crate::component::MegaMenu;

new_handle!(COMPONENT_MEGAITEM);

type Label = OneComponent<L10n>;
type Content = OneComponent<Html>;

pub type MegaItemPath = fn(cx: &Context) -> &str;

#[derive(Default)]
pub enum MegaItemType {
    #[default]
    Void,
    Label(Label),
    Link(Label, MegaItemPath),
    LinkBlank(Label, MegaItemPath),
    Html(Content),
    Submenu(Label, MegaMenu),
    Separator,
}

// MegaMenuItem.

#[rustfmt::skip]
#[derive(Default)]
pub struct MegaItem {
    weight    : Weight,
    renderable: Renderable,
    item_type : MegaItemType,
}

impl ComponentTrait for MegaItem {
    fn new() -> Self {
        MegaItem::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_MEGAITEM
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.item_type() {
            MegaItemType::Void => PrepareMarkup::None,
            MegaItemType::Label(label) => PrepareMarkup::With(html! {
                li class="link" { a href="#" { (label.prepare(cx)) } }
            }),
            MegaItemType::Link(label, path) => PrepareMarkup::With(html! {
                li class="link" { a href=(path(cx)) { (label.prepare(cx)) } }
            }),
            MegaItemType::LinkBlank(label, path) => PrepareMarkup::With(html! {
                li class="link" { a href=(path(cx)) target="_blank" { (label.prepare(cx)) } }
            }),
            MegaItemType::Html(content) => PrepareMarkup::With(html! {
                li class="html" { (content.prepare(cx)) }
            }),
            MegaItemType::Submenu(label, menu) => PrepareMarkup::With(html! {
                li class="submenu" {
                    a href="#" { (label.prepare(cx)) }
                    ul {
                        (menu.items().prepare(cx))
                    }
                }
            }),
            MegaItemType::Separator => PrepareMarkup::With(html! {
                li class="separator" { }
            }),
        }
    }
}

impl MegaItem {
    pub fn label(label: L10n) -> Self {
        MegaItem {
            item_type: MegaItemType::Label(OneComponent::new_with(label)),
            ..Default::default()
        }
    }

    pub fn link(label: L10n, path: MegaItemPath) -> Self {
        MegaItem {
            item_type: MegaItemType::Link(OneComponent::new_with(label), path),
            ..Default::default()
        }
    }

    pub fn link_blank(label: L10n, path: MegaItemPath) -> Self {
        MegaItem {
            item_type: MegaItemType::LinkBlank(OneComponent::new_with(label), path),
            ..Default::default()
        }
    }

    pub fn html(content: Html) -> Self {
        MegaItem {
            item_type: MegaItemType::Html(OneComponent::new_with(content)),
            ..Default::default()
        }
    }

    pub fn submenu(label: L10n, menu: MegaMenu) -> Self {
        MegaItem {
            item_type: MegaItemType::Submenu(OneComponent::new_with(label), menu),
            ..Default::default()
        }
    }

    pub fn separator() -> Self {
        MegaItem {
            item_type: MegaItemType::Separator,
            ..Default::default()
        }
    }

    // MegaItem BUILDER.

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

    // MegaItem GETTERS.

    pub fn item_type(&self) -> &MegaItemType {
        &self.item_type
    }
}
