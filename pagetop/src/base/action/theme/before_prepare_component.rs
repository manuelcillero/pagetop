use crate::prelude::*;

use crate::base::action::FnActionWithComponent;

pub struct BeforePrepare<C: ComponentTrait> {
    f: FnActionWithComponent<C>,
    theme_type_id: Option<TypeId>,
    referer_type_id: Option<TypeId>,
}

impl<C: ComponentTrait> ActionTrait for BeforePrepare<C> {
    fn theme_type_id(&self) -> Option<TypeId> {
        self.theme_type_id
    }

    fn referer_type_id(&self) -> Option<TypeId> {
        self.referer_type_id
    }
}

impl<C: ComponentTrait> BeforePrepare<C> {
    pub fn new(theme: ThemeRef, f: FnActionWithComponent<C>) -> Self {
        BeforePrepare {
            f,
            theme_type_id: Some(theme.type_id()),
            referer_type_id: Some(TypeId::of::<C>()),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
        dispatch_actions(
            &ActionKey::new(
                TypeId::of::<Self>(),
                Some(cx.theme().type_id()),
                Some(TypeId::of::<C>()),
                None,
            ),
            |action: &Self| (action.f)(component, cx),
        );
    }
}
