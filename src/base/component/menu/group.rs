use crate::prelude::*;

use super::Element;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Group {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    elements  : VectorComponents<Element>,
}

impl ComponentTrait for Group {
    fn new() -> Self {
        Group::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
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
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[rustfmt::skip]
    pub fn add_element(mut self, element: Element) -> Self {
        self.elements.alter_value(TypedOp::Add(TypedComponent::with(element)));
        self
    }

    #[fn_builder]
    pub fn alter_elements(&mut self, op: TypedOp<Element>) -> &mut Self {
        self.elements.alter_value(op);
        self
    }

    // Group GETTERS.

    pub fn elements(&self) -> &VectorComponents<Element> {
        &self.elements
    }
}
