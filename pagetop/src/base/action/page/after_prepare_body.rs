use crate::prelude::*;

use super::FnActionPage;

new_handle!(ACTION_AFTER_PREPARE_BODY for Crate);

pub struct AfterPrepareBody {
    action: Option<FnActionPage>,
    weight: Weight,
}

impl ActionTrait for AfterPrepareBody {
    fn new() -> Self {
        AfterPrepareBody {
            action: None,
            weight: 0,
        }
    }

    fn handle(&self) -> Handle {
        ACTION_AFTER_PREPARE_BODY
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AfterPrepareBody {
    pub fn with(action: FnActionPage) -> Self {
        AfterPrepareBody {
            action: Some(action),
            weight: 0,
        }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    pub(crate) fn run(&self, page: &mut Page) {
        if let Some(action) = self.action {
            action(page)
        }
    }
}

#[inline(always)]
pub(crate) fn run_actions_after_prepare_body(page: &mut Page) {
    run_actions(ACTION_AFTER_PREPARE_BODY, |action| {
        action_ref::<AfterPrepareBody>(&**action).run(page)
    });
}
