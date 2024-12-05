use crate::prelude::*;

use crate::base::action::FnActionWithPage;

pub struct BeforeRenderBody {
    f: FnActionWithPage,
    layout_type_id: Option<TypeId>,
}

impl ActionTrait for BeforeRenderBody {
    fn layout_type_id(&self) -> Option<TypeId> {
        self.layout_type_id
    }
}

impl BeforeRenderBody {
    pub fn new(layout: LayoutRef, f: FnActionWithPage) -> Self {
        BeforeRenderBody {
            f,
            layout_type_id: Some(layout.type_id()),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions(
            &ActionKey::new(
                TypeId::of::<Self>(),
                Some(page.context().layout().type_id()),
                None,
                None,
            ),
            |action: &Self| (action.f)(page),
        );
    }
}
