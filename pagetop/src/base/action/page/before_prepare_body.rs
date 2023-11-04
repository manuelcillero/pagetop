use crate::prelude::*;

use super::FnActionPage;

pub struct BeforePrepareBody {
    f: Option<FnActionPage>,
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
        BeforePrepareBody {
            f: Some(f),
            weight: 0,
        }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions((ACTION_BEFORE_PREPARE_BODY, None, None), |action| {
            if let Some(f) = action_ref::<BeforePrepareBody>(&**action).f {
                f(page)
            }
        });
    }
}
