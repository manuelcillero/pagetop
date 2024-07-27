use crate::prelude::*;

pub type FnBeforePrepareBody = fn(page: &mut Page);

pub struct BeforePrepareBody {
    f: FnBeforePrepareBody,
    weight: Weight,
}

impl ActionTrait for BeforePrepareBody {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl BeforePrepareBody {
    pub fn new(f: FnBeforePrepareBody) -> Self {
        BeforePrepareBody { f, weight: 0 }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions(
            &ActionKey::new(TypeId::of::<Self>(), None, None, None),
            |action: &Self| (action.f)(page),
        );
    }
}
