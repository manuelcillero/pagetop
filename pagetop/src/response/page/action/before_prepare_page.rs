use super::ActionPage;
use crate::core::action::{action_ref, run_actions, ActionTrait, AnyAction};
use crate::response::page::Page;
use crate::{use_handle, Handle};

use_handle!(ACTION_BEFORE_PREPARE_PAGE for Action);

pub struct ActionBeforePreparePage {
    action: Option<ActionPage>,
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
    pub fn with_action(mut self, action: ActionPage) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }

    pub(crate) fn run(&self, page: &mut Page) {
        if let Some(action) = self.action {
            action(page)
        }
    }
}

#[inline(always)]
pub(crate) fn run_actions_before_prepare_page(page: &mut Page) {
    run_actions(ACTION_BEFORE_PREPARE_PAGE, |action| {
        action_ref::<ActionBeforePreparePage>(&**action).run(page)
    });
}
