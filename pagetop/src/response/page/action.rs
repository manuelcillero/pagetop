use crate::api::action::{ActionTrait, AnyAction};
use crate::api::component::{ComponentTrait, PageAssets};
use super::Page;

pub enum TypeAction {
    BeforeRenderPage(fn(&mut Page)),
    BeforeRenderComponent(fn(&mut dyn ComponentTrait, &mut PageAssets)),
    None,
}

pub struct PageAction {
    action: TypeAction,
    weight: i8,
}

impl ActionTrait for PageAction {
    fn new() -> Self {
        PageAction {
            action: TypeAction::None,
            weight: 0,
        }
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn as_ref_any(&self) -> &dyn AnyAction {
        self
    }
}

impl PageAction {
    pub fn new(action: TypeAction) -> Self {
        PageAction {
            action,
            weight: 0,
        }
    }

    pub fn new_with_weight(action: TypeAction, weight: i8) -> Self {
        PageAction {
            action,
            weight,
        }
    }

    pub fn before_render_page(&self, page: &mut Page) {
        if let TypeAction::BeforeRenderPage(f) = self.action {
            f(page)
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
