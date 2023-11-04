use crate::prelude::*;

use super::FnActionPage;

pub struct BeforePrepareBody {
    f: FnActionPage,
    weight: Weight,
}

impl_handle!(ACTION_BEFORE_PREPARE_BODY for BeforePrepareBody);

impl ActionTrait for BeforePrepareBody {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl BeforePrepareBody {
    pub fn with(f: FnActionPage) -> Self {
        BeforePrepareBody { f, weight: 0 }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions((Self::static_handle(), None, None), |action| {
            (action_ref::<BeforePrepareBody>(&**action).f)(page)
        });
    }
}
