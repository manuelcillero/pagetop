use crate::prelude::*;

use super::FnActionPage;

pub struct BeforePrepareBody {
    f: FnActionPage,
    weight: Weight,
}

impl ActionTrait for BeforePrepareBody {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl BeforePrepareBody {
    pub fn new(f: FnActionPage) -> Self {
        BeforePrepareBody { f, weight: 0 }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions((TypeId::of::<Self>(), None, None), |action: &Self| {
            (action.f)(page)
        });
    }
}
