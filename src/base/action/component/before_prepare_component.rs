use crate::prelude::*;

use super::FnActionComponent;

pub struct BeforePrepareComponent<C: ComponentTrait> {
    f: FnActionComponent<C>,
    referer_type_id: Option<TypeId>,
    referer_id: OptionId,
    weight: Weight,
}

impl<C: ComponentTrait> ActionTrait for BeforePrepareComponent<C> {
    fn referer_type_id(&self) -> Option<TypeId> {
        self.referer_type_id
    }

    fn referer_id(&self) -> Option<String> {
        self.referer_id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl<C: ComponentTrait> BeforePrepareComponent<C> {
    pub fn new(f: FnActionComponent<C>) -> Self {
        BeforePrepareComponent {
            f,
            referer_type_id: Some(TypeId::of::<C>()),
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
            (TypeId::of::<Self>(), Some(TypeId::of::<C>()), referer_id),
            |action| (action_ref::<BeforePrepareComponent<C>>(&**action).f)(component, cx),
        );
    }
}
