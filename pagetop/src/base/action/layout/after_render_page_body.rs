use crate::prelude::*;

use crate::base::action::FnActionWithPage;

pub struct AfterRenderBody {
    f: FnActionWithPage,
    layout_type_id: Option<UniqueId>,
}

impl ActionTrait for AfterRenderBody {
    fn layout_type_id(&self) -> Option<UniqueId> {
        self.layout_type_id
    }
}

impl AfterRenderBody {
    pub fn new(layout: LayoutRef, f: FnActionWithPage) -> Self {
        AfterRenderBody {
            f,
            layout_type_id: Some(layout.type_id()),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                Some(page.context().layout().type_id()),
                None,
                None,
            ),
            |action: &Self| (action.f)(page),
        );
    }
}
