use crate::prelude::*;

use super::Element;

new_handle!(COMPONENT_BASE_MENU_GROUP);

type Elements = TypedComponents<Element>;

#[rustfmt::skip]
#[derive(Default)]
pub struct Group {
    weight    : Weight,
    renderable: Renderable,
    id        : OptionId,
    elements  : Elements,
}

impl ComponentTrait for Group {
    fn new() -> Self {
        Group::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_MENU_GROUP
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
                (self.elements().prepare(cx))
            }
        })
    }
}

impl Group {
    // Group BUILDER.

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
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    pub fn with_element(mut self, element: Element) -> Self {
        self.elements
            .alter(TypedOp::Add(TypedComponent::with(element)));
        self
    }

    #[fn_builder]
    pub fn alter_elements(&mut self, op: TypedOp<Element>) -> &mut Self {
        self.elements.alter(op);
        self
    }

    // Group GETTERS.

    pub fn elements(&self) -> &Elements {
        &self.elements
    }
}
