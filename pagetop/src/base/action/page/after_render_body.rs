use crate::prelude::*;

use crate::base::action::FnActionWithPage;

pub struct AfterRenderBody {
    f: FnActionWithPage,
    weight: Weight,
}

impl ActionTrait for AfterRenderBody {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AfterRenderBody {
    pub fn new(f: FnActionWithPage) -> Self {
        AfterRenderBody { f, weight: 0 }
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
