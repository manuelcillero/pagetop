use crate::prelude::*;

use super::Submenu;

new_handle!(COMPONENT_BASE_MENU_ELEMENT);

type Content = TypedComponent<Html>;
type SubmenuItems = TypedComponent<Submenu>;

#[derive(Default)]
pub enum ElementType {
    #[default]
    Void,
    Html(Content),
    Submenu(SubmenuItems),
}

// Element.

#[rustfmt::skip]
#[derive(Default)]
pub struct Element {
    weight      : Weight,
    renderable  : Renderable,
    element_type: ElementType,
}

impl ComponentTrait for Element {
    fn new() -> Self {
        Element::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_MENU_ELEMENT
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.element_type() {
            ElementType::Void => PrepareMarkup::None,
            ElementType::Html(content) => PrepareMarkup::With(html! {
                (content.prepare(cx))
            }),
            ElementType::Submenu(submenu) => PrepareMarkup::With(html! {
                (submenu.prepare(cx))
            }),
        }
    }
}

impl Element {
    pub fn html(content: Html) -> Self {
        Element {
            element_type: ElementType::Html(Content::with(content)),
            ..Default::default()
        }
    }

    pub fn submenu(submenu: Submenu) -> Self {
        Element {
            element_type: ElementType::Submenu(SubmenuItems::with(submenu)),
            ..Default::default()
        }
    }

    // Element BUILDER.

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

    // Element GETTERS.

    pub fn element_type(&self) -> &ElementType {
        &self.element_type
    }
}
