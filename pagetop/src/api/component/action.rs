use crate::api::action::{ActionTrait, AnyAction};
use super::{Assets, ComponentTrait};

pub const BEFORE_RENDER_COMPONENT_ACTION: &str = "pagetop::action::before_render_component";

pub struct BeforeRenderComponentAction {
    action: Option<fn(&mut dyn ComponentTrait, &mut Assets)>,
    weight: isize,
}

impl ActionTrait for BeforeRenderComponentAction {
    fn new() -> Self {
        BeforeRenderComponentAction {
            action: None,
            weight: 0,
        }
    }

    fn handler(&self) -> &'static str {
        BEFORE_RENDER_COMPONENT_ACTION
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn as_ref_any(&self) -> &dyn AnyAction {
        self
    }
}

impl BeforeRenderComponentAction {
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
