use crate::prelude::*;

use super::FnAction;

pub struct BeforePrepareComponent<C: ComponentTrait> {
    action: Option<FnAction<C>>,
    referer: Option<Handle>,
    weight: Weight,
}

impl_handle!(ACTION_BEFORE_PREPARE_COMPONENT for BeforePrepareComponent<ComponentTrait>);

impl<C: ComponentTrait> ActionTrait for BeforePrepareComponent<C> {
    fn new() -> Self {
        BeforePrepareComponent {
            action: None,
            referer: Some(C::static_handle()),
            weight: 0,
        }
    }

    fn referer_handle(&self) -> Option<Handle> {
        self.referer
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl<C: ComponentTrait> BeforePrepareComponent<C> {
    pub fn with(action: FnAction<C>) -> Self {
        BeforePrepareComponent {
            action: Some(action),
            referer: Some(C::static_handle()),
            weight: 0,
        }
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    pub(crate) fn run(&self, component: &mut C, cx: &mut Context) {
        if let Some(action) = self.action {
            action(component, cx)
        }
    }
}

#[inline(always)]
pub(crate) fn run_actions_before_prepare_component<C: ComponentTrait>(
    component: &mut C,
    cx: &mut Context,
) {
    run_actions(
        (ACTION_BEFORE_PREPARE_COMPONENT, Some(component.handle())),
        |action| action_ref::<BeforePrepareComponent<C>>(&**action).run(component, cx),
    );
}