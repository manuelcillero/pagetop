use crate::prelude::*;

pub type FnIsRenderable<C> = fn(component: &C, cx: &mut Context) -> bool;

pub struct IsRenderable<C: ComponentTrait> {
    f: FnIsRenderable<C>,
    referer_type_id: Option<UniqueId>,
    referer_id: OptionId,
    weight: Weight,
}

impl<C: ComponentTrait> ActionTrait for IsRenderable<C> {
    fn referer_type_id(&self) -> Option<UniqueId> {
        self.referer_type_id
    }

    fn referer_id(&self) -> Option<String> {
        self.referer_id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl<C: ComponentTrait> IsRenderable<C> {
    pub fn new(f: FnIsRenderable<C>) -> Self {
        IsRenderable {
            f,
            referer_type_id: Some(UniqueId::of::<C>()),
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
    pub(crate) fn dispatch(component: &C, cx: &mut Context) -> bool {
        let mut renderable = true;
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                None,
                Some(UniqueId::of::<C>()),
                None,
            ),
            |action: &Self| {
                if renderable && !(action.f)(component, cx) {
                    renderable = false;
                }
            },
        );
        if renderable {
            if let Some(id) = component.id() {
                dispatch_actions(
                    &ActionKey::new(
                        UniqueId::of::<Self>(),
                        None,
                        Some(UniqueId::of::<C>()),
                        Some(id),
                    ),
                    |action: &Self| {
                        if renderable && !(action.f)(component, cx) {
                            renderable = false;
                        }
                    },
                );
            }
        }
        renderable
    }
}
