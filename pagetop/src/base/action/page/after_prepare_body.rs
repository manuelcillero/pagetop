use crate::prelude::*;
use crate::BaseHandle;

use super::FnActionPage;

#[derive(BaseHandle)]
pub struct AfterPrepareBody {
    f: FnActionPage,
    weight: Weight,
}

impl ActionTrait for AfterPrepareBody {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AfterPrepareBody {
    pub fn new(f: FnActionPage) -> Self {
        AfterPrepareBody { f, weight: 0 }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions((Self::static_handle(), None, None), |action| {
            (action_ref::<AfterPrepareBody>(&**action).f)(page)
        });
    }
}
