use crate::prelude::*;

use super::Submenu;

type Content = TypedComponent<Html>;
type SubmenuItems = TypedComponent<Submenu>;

#[derive(AutoDefault)]
pub enum ElementType {
    #[default]
    Void,
    Html(Content),
    Submenu(SubmenuItems),
}

// Element.

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Element {
    element_type: ElementType,
}

impl ComponentTrait for Element {
    fn new() -> Self {
        Element::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.element_type() {
            ElementType::Void => PrepareMarkup::None,
            ElementType::Html(content) => PrepareMarkup::With(html! {
                (content.render(cx))
            }),
            ElementType::Submenu(submenu) => PrepareMarkup::With(html! {
                (submenu.render(cx))
            }),
        }
    }
}

impl Element {
    pub fn html(content: Html) -> Self {
        Element {
            element_type: ElementType::Html(Content::with(content)),
        }
    }

    pub fn submenu(submenu: Submenu) -> Self {
        Element {
            element_type: ElementType::Submenu(SubmenuItems::with(submenu)),
        }
    }

    // Element GETTERS.

    pub fn element_type(&self) -> &ElementType {
        &self.element_type
    }
}
