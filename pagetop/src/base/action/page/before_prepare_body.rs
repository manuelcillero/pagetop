use crate::prelude::*;

use super::FnActionPage;

pub struct BeforePrepareBody {
    action: Option<FnActionPage>,
    weight: Weight,
}

impl_handle!(ACTION_BEFORE_PREPARE_BODY for BeforePrepareBody);

impl ActionTrait for BeforePrepareBody {
    fn new() -> Self {
        BeforePrepareBody {
            action: None,
            weight: 0,
        }
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl BeforePrepareBody {
    pub fn with(action: FnActionPage) -> Self {
        BeforePrepareBody {
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
pub(crate) fn run_actions_before_prepare_body(page: &mut Page) {
    run_actions((ACTION_BEFORE_PREPARE_BODY, None), |action| {
        action_ref::<BeforePrepareBody>(&**action).run(page)
    });
}
