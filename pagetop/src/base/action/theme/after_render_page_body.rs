use crate::prelude::*;

use crate::base::action::FnActionWithPage;

pub struct AfterRenderBody {
    f: FnActionWithPage,
    theme_type_id: Option<UniqueId>,
}

impl ActionTrait for AfterRenderBody {
    fn theme_type_id(&self) -> Option<UniqueId> {
        self.theme_type_id
    }
}

impl AfterRenderBody {
    pub fn new(theme: ThemeRef, f: FnActionWithPage) -> Self {
        AfterRenderBody {
            f,
            theme_type_id: Some(theme.type_id()),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(page: &mut Page) {
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                Some(page.context().theme().type_id()),
                None,
                None,
            ),
            |action: &Self| (action.f)(page),
        );
    }
}
