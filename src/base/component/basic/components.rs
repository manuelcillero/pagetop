use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Components(MixedComponents);

impl ComponentTrait for Components {
    fn new() -> Self {
        Components::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(self.components().render(cx))
    }
}

impl Components {
    // Components BUILDER.

    #[fn_builder]
    pub fn alter_components(&mut self, op: AnyOp) -> &mut Self {
        self.0.alter_value(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.0.alter_value(AnyOp::Add(AnyComponent::with(component)));
        self
    }

    // Components GETTERS.

    pub fn components(&self) -> &MixedComponents {
        &self.0
    }
}
