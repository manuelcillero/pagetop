use crate::api::action::{ActionTrait, AnyAction};
use super::{Assets, ComponentTrait};

pub struct ActionBeforeRenderComponent {
    action: Option<fn(&mut dyn ComponentTrait, &mut Assets)>,
    weight: isize,
}

impl ActionTrait for ActionBeforeRenderComponent {
    fn new() -> Self {
        ActionBeforeRenderComponent {
            action: None,
            weight: 0,
        }
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn as_ref_any(&self) -> &dyn AnyAction {
        self
    }
}

impl ActionBeforeRenderComponent {
    pub fn with_action(mut self, action: fn(&mut dyn ComponentTrait, &mut Assets)) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }

    pub fn run(&self, component: &mut dyn ComponentTrait, assets: &mut Assets) {
        if let Some(action) = self.action {
            action(component, assets)
        }
    }
}
