use crate::api::action::{ActionTrait, AnyAction};
use super::Page;

pub struct ActionBeforeRenderPage {
    action: Option<fn(&mut Page)>,
    weight: isize,
}

impl ActionTrait for ActionBeforeRenderPage {
    fn new() -> Self {
        ActionBeforeRenderPage {
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

impl ActionBeforeRenderPage {
    pub fn with_action(mut self, action: fn(&mut Page)) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }

    pub fn run(&self, page: &mut Page) {
        if let Some(action) = self.action {
            action(page)
        }
    }
}
