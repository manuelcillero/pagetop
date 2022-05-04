use crate::api::action::{ActionTrait, AnyAction};
use super::{ComponentTrait, PageAssets};

pub enum TypeAction {
    BeforeRenderComponent(fn(&mut dyn ComponentTrait, &mut PageAssets)),
    None,
}

pub struct ComponentAction {
    action: TypeAction,
    weight: isize,
}

impl ActionTrait for ComponentAction {
    fn new() -> Self {
        ComponentAction {
            action: TypeAction::None,
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

impl ComponentAction {
    pub fn new(action: TypeAction) -> Self {
        ComponentAction {
            action,
            weight: 0,
        }
    }

    pub fn new_with_weight(action: TypeAction, weight: isize) -> Self {
        ComponentAction {
            action,
            weight,
        }
    }

    pub fn before_render_component(
        &self,
        component: &mut dyn ComponentTrait,
        assets: &mut PageAssets)
    {
        if let TypeAction::BeforeRenderComponent(f) = self.action {
            f(component, assets)
        }
    }
}
