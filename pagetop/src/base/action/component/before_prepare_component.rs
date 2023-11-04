use crate::prelude::*;

use super::FnAction;

pub struct BeforePrepareComponent<C: ComponentTrait> {
    f: Option<FnAction<C>>,
    referer_handle: Option<Handle>,
    referer_id: OptionId,
    weight: Weight,
}

impl_handle!(ACTION_BEFORE_PREPARE_COMPONENT for BeforePrepareComponent<ComponentTrait>);

impl<C: ComponentTrait> ActionTrait for BeforePrepareComponent<C> {
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

impl<C: ComponentTrait> BeforePrepareComponent<C> {
    pub fn with(f: FnAction<C>) -> Self {
        BeforePrepareComponent {
            f: Some(f),
            referer_handle: Some(C::static_handle()),
            referer_id: OptionId::default(),
            weight: 0,
        }
    }

    pub fn filtering_id(mut self, id: impl Into<String>) -> Self {
        self.referer_id.alter_value(id);
        self
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    #[inline(always)]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context, id: Option<String>) {
        dispatch_actions(
            (
                ACTION_BEFORE_PREPARE_COMPONENT,
                Some(component.handle()),
                id,
            ),
            |action| {
                if let Some(f) = action_ref::<BeforePrepareComponent<C>>(&**action).f {
                    f(component, cx)
                }
            },
        );
    }
}
