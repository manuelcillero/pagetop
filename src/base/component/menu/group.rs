use crate::prelude::*;

use super::Element;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Group {
    id      : OptionId,
    elements: MixedComponents,
}

impl ComponentTrait for Group {
    fn new() -> Self {
        Group::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="menu-group" {
                (self.elements().render(cx))
            }
        })
    }
}

impl Group {
    // Group BUILDER.

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_elements(&mut self, op: TypedOp<Element>) -> &mut Self {
        self.elements.alter_typed(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_element(mut self, element: Element) -> Self {
        self.elements.alter_value(AnyOp::Add(AnyComponent::with(element)));
        self
    }

    // Group GETTERS.

    pub fn elements(&self) -> &MixedComponents {
        &self.elements
    }
}
