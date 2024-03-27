use crate::prelude::*;

pub type FnAfterPrepareBody = fn(page: &mut Page);

pub struct AfterPrepareBody {
    f: FnAfterPrepareBody,
    weight: Weight,
}

impl ActionTrait for AfterPrepareBody {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AfterPrepareBody {
    pub fn new(f: FnAfterPrepareBody) -> Self {
        AfterPrepareBody { f, weight: 0 }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions(
            ActionKey::new(TypeId::of::<Self>(), None, None, None),
            |action: &Self| (action.f)(page),
        );
    }
}
