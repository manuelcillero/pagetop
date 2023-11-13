use crate::prelude::*;

use super::Element;

#[rustfmt::skip]
#[derive(Default)]
pub struct Group {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    elements  : TypedComponents<Element>,
}

impl_handle!(COMPONENT_BASE_MENU_GROUP for Group);

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
        self.elements.alter_value(ArcTypedOp::Add(ArcTypedComponent::new(element)));
        self
    }

    #[fn_builder]
    pub fn alter_elements(&mut self, op: ArcTypedOp<Element>) -> &mut Self {
        self.elements.alter_value(op);
        self
    }

    // Group GETTERS.

    pub fn elements(&self) -> &TypedComponents<Element> {
        &self.elements
    }
}
