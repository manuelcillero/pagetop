use crate::core::action::{ActionTrait, AnyAction};
use crate::response::page::Page;
use crate::{define_handle, Handle};

define_handle!(ACTION_BEFORE_RENDER_PAGE);

type Action = fn(&mut Page);

pub struct ActionBeforeRenderPage {
    action: Option<Action>,
    weight: isize,
}

impl ActionTrait for ActionBeforeRenderPage {
    fn new() -> Self {
        ActionBeforeRenderPage {
            action: None,
            weight: 0,
        }
    }

    fn handle(&self) -> Handle {
        ACTION_BEFORE_RENDER_PAGE
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn as_ref_any(&self) -> &dyn AnyAction {
        self
    }
}

impl ActionBeforeRenderPage {
    pub fn with_action(mut self, action: Action) -> Self {
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
