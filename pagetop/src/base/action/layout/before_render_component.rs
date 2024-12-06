use crate::prelude::*;

use crate::base::action::FnActionWithComponent;

pub struct BeforeRender<C: ComponentTrait> {
    f: FnActionWithComponent<C>,
    layout_type_id: Option<TypeId>,
    referer_type_id: Option<TypeId>,
}

impl<C: ComponentTrait> ActionTrait for BeforeRender<C> {
    fn layout_type_id(&self) -> Option<TypeId> {
        self.layout_type_id
    }

    fn referer_type_id(&self) -> Option<TypeId> {
        self.referer_type_id
    }
}

impl<C: ComponentTrait> BeforeRender<C> {
    pub fn new(layout: LayoutRef, f: FnActionWithComponent<C>) -> Self {
        BeforeRender {
            f,
            layout_type_id: Some(layout.type_id()),
            referer_type_id: Some(TypeId::of::<C>()),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
        dispatch_actions(
            &ActionKey::new(
                TypeId::of::<Self>(),
                Some(cx.layout().type_id()),
                Some(TypeId::of::<C>()),
                None,
            ),
            |action: &Self| (action.f)(component, cx),
        );
    }
}