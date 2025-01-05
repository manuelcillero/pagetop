use pagetop::prelude::*;

use crate::bs::navbar;

#[derive(AutoDefault)]
pub enum ElementType {
    #[default]
    None,
    Nav(navbar::Nav),
    Text(L10n),
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
            ElementType::None => PrepareMarkup::None,
            ElementType::Nav(_nav) => PrepareMarkup::With(html! {
                span class="navbar-text" {
                    ("Prueba")
                }
            }),
            ElementType::Text(label) => PrepareMarkup::With(html! {
                span class="navbar-text" {
                    (label.escaped(cx.langid()))
                }
            }),
        }
    }
}

impl Element {
    pub fn nav(nav: navbar::Nav) -> Self {
        Element {
            element_type: ElementType::Nav(nav),
            ..Default::default()
        }
    }

    pub fn text(label: L10n) -> Self {
        Element {
            element_type: ElementType::Text(label),
            ..Default::default()
        }
    }

    // Element GETTERS.

    pub fn element_type(&self) -> &ElementType {
        &self.element_type
    }
}
