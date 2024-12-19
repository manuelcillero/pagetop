use crate::prelude::*;

use crate::base::action::FnActionWithComponent;

pub struct AfterRender<C: ComponentTrait> {
    f: FnActionWithComponent<C>,
    theme_type_id: Option<UniqueId>,
    referer_type_id: Option<UniqueId>,
}

impl<C: ComponentTrait> ActionTrait for AfterRender<C> {
    fn theme_type_id(&self) -> Option<UniqueId> {
        self.theme_type_id
    }

    fn referer_type_id(&self) -> Option<UniqueId> {
        self.referer_type_id
    }
}

impl<C: ComponentTrait> AfterRender<C> {
    pub fn new(theme: ThemeRef, f: FnActionWithComponent<C>) -> Self {
        AfterRender {
            f,
            theme_type_id: Some(theme.type_id()),
            referer_type_id: Some(UniqueId::of::<C>()),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                Some(cx.theme().type_id()),
                Some(UniqueId::of::<C>()),
                None,
            ),
            |action: &Self| (action.f)(component, cx),
        );
    }
}
