use crate::core::action::{action_ref, run_actions, ActionTrait};
use crate::response::page::action::FnActionPage;
use crate::response::page::Page;
use crate::{new_handle, Handle, Weight};

new_handle!(ACTION_BEFORE_PREPARE_BODY for Action);

pub struct ActionBeforePrepareBody {
    action: Option<FnActionPage>,
    weight: Weight,
}

impl ActionTrait for ActionBeforePrepareBody {
    fn new() -> Self {
        ActionBeforePrepareBody {
            action: None,
            weight: 0,
        }
    }

    fn handle(&self) -> Handle {
        ACTION_BEFORE_PREPARE_BODY
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl ActionBeforePrepareBody {
    pub fn with_action(mut self, action: FnActionPage) -> Self {
        self.action = Some(action);
        self
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
pub(crate) fn run_actions_before_prepare_body(page: &mut Page) {
    run_actions(ACTION_BEFORE_PREPARE_BODY, |action| {
        action_ref::<ActionBeforePrepareBody>(&**action).run(page)
    });
}
