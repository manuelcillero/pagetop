use crate::core::component::{AnyComponent, ComponentTrait};

#[derive(Default)]
pub struct DefaultComponent;

impl ComponentTrait for DefaultComponent {
    fn new() -> Self {
        DefaultComponent::default()
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}
