use crate::prelude::*;

use super::FnAction;

pub struct AfterPrepareComponent<C: ComponentTrait> {
    f: FnAction<C>,
    referer_handle: Option<Handle>,
    referer_id: OptionId,
    weight: Weight,
}

impl_handle!(ACTION_AFTER_PREPARE_COMPONENT for AfterPrepareComponent<ComponentTrait>);

impl<C: ComponentTrait> ActionTrait for AfterPrepareComponent<C> {
    fn referer_handle(&self) -> Option<Handle> {
        self.referer_handle
    }

    fn referer_id(&self) -> Option<String> {
        self.referer_id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl<C: ComponentTrait> AfterPrepareComponent<C> {
    pub fn new(f: FnAction<C>) -> Self {
        AfterPrepareComponent {
            f,
            referer_handle: Some(C::static_handle()),
            referer_id: OptionId::default(),
            weight: 0,
        }
    }

    pub fn filter_by_referer_id(mut self, id: impl Into<String>) -> Self {
        self.referer_id.alter_value(id);
        self
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context, referer_id: Option<String>) {
        dispatch_actions(
            (Self::static_handle(), Some(component.handle()), referer_id),
            |action| (action_ref::<AfterPrepareComponent<C>>(&**action).f)(component, cx),
        );
    }
}
