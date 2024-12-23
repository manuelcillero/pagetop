use crate::prelude::*;

use crate::base::action::FnActionWithPage;

pub struct BeforeRenderBody {
    f: FnActionWithPage,
    weight: Weight,
}

impl ActionTrait for BeforeRenderBody {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl BeforeRenderBody {
    pub fn new(f: FnActionWithPage) -> Self {
        BeforeRenderBody { f, weight: 0 }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None, None),
            |action: &Self| (action.f)(page),
        );
    }
}
