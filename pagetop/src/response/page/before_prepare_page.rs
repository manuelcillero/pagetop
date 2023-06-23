use crate::core::action::{ActionTrait, AnyAction};
use crate::response::page::Page;
use crate::{use_handle, Handle};

use_handle!(ACTION_BEFORE_PREPARE_PAGE);

type Action = fn(&mut Page);

pub struct ActionBeforePreparePage {
    action: Option<Action>,
    weight: isize,
}

impl ActionTrait for ActionBeforePreparePage {
    fn new() -> Self {
        ActionBeforePreparePage {
            action: None,
            weight: 0,
        }
    }

    fn handle(&self) -> Handle {
        ACTION_BEFORE_PREPARE_PAGE
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn as_ref_any(&self) -> &dyn AnyAction {
        self
    }
}

impl ActionBeforePreparePage {
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
