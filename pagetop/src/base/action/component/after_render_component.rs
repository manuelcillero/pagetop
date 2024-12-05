use crate::prelude::*;

use crate::base::action::FnActionWithComponent;

pub struct AfterRender<C: ComponentTrait> {
    f: FnActionWithComponent<C>,
    referer_type_id: Option<TypeId>,
    referer_id: OptionId,
    weight: Weight,
}

impl<C: ComponentTrait> ActionTrait for AfterRender<C> {
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

impl<C: ComponentTrait> AfterRender<C> {
    pub fn new(f: FnActionWithComponent<C>) -> Self {
        AfterRender {
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
    #[allow(clippy::inline_always)]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
        dispatch_actions(
            &ActionKey::new(TypeId::of::<Self>(), None, Some(TypeId::of::<C>()), None),
            |action: &Self| (action.f)(component, cx),
        );
        if let Some(id) = component.id() {
            dispatch_actions(
                &ActionKey::new(
                    TypeId::of::<Self>(),
                    None,
                    Some(TypeId::of::<C>()),
                    Some(id),
                ),
                |action: &Self| (action.f)(component, cx),
            );
        }
    }
}
