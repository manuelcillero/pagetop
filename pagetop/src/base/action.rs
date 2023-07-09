pub mod page;

pub mod block {
    crate::actions_for_component!(Block);
}

#[macro_export]
macro_rules! actions_for_component {
    ( $Component:ty ) => {
        $crate::paste! {
            use $crate::prelude::*;

            pub type [<Action $Component>] = fn(component: &$Component, cx: &mut Context);

            // *************************************************************************************
            // ACTION BEFORE PREPARE COMPONENT
            // *************************************************************************************

            $crate::use_handle!([<ACTION_BEFORE_PREPARE_ $Component:upper>] for Action);

            pub struct [<BeforePrepare $Component>] {
                action: Option<[<Action $Component>]>,
                weight: isize,
            }

            impl ActionTrait for [<BeforePrepare $Component>] {
                fn new() -> Self {
                    [<BeforePrepare $Component>] {
                        action: None,
                        weight: 0,
                    }
                }

                fn handle(&self) -> Handle {
                    [<ACTION_BEFORE_PREPARE_ $Component:upper>]
                }

                fn weight(&self) -> isize {
                    self.weight
                }

                fn as_ref_any(&self) -> &dyn AnyAction {
                    self
                }
            }

            impl [<BeforePrepare $Component>] {
                #[allow(dead_code)]
                pub fn with_action(mut self, action: [<Action $Component>]) -> Self {
                    self.action = Some(action);
                    self
                }

                #[allow(dead_code)]
                pub fn with_weight(mut self, weight: isize) -> Self {
                    self.weight = weight;
                    self
                }

                pub(crate) fn run(&self, component: &mut $Component, cx: &mut Context) {
                    if let Some(action) = self.action {
                        action(component, cx)
                    }
                }
            }

            #[inline(always)]
            pub(crate) fn [<run_actions_before_prepare_ $Component:lower>](
                component: &mut $Component,
                cx: &mut Context
            ) {
                run_actions([<ACTION_BEFORE_PREPARE_ $Component:upper>], |action|
                    action_ref::<[<BeforePrepare $Component>]>(&**action)
                        .run(component, cx)
                );
            }

            // *************************************************************************************
            // ACTION AFTER PREPARE COMPONENT
            // *************************************************************************************

            $crate::use_handle!([<ACTION_AFTER_PREPARE_ $Component:upper>] for Action);

            pub struct [<AfterPrepare $Component>] {
                action: Option<[<Action $Component>]>,
                weight: isize,
            }

            impl ActionTrait for [<AfterPrepare $Component>] {
                fn new() -> Self {
                    [<AfterPrepare $Component>] {
                        action: None,
                        weight: 0,
                    }
                }

                fn handle(&self) -> Handle {
                    [<ACTION_AFTER_PREPARE_ $Component:upper>]
                }

                fn weight(&self) -> isize {
                    self.weight
                }

                fn as_ref_any(&self) -> &dyn AnyAction {
                    self
                }
            }

            impl [<AfterPrepare $Component>] {
                #[allow(dead_code)]
                pub fn with_action(mut self, action: [<Action $Component>]) -> Self {
                    self.action = Some(action);
                    self
                }

                #[allow(dead_code)]
                pub fn with_weight(mut self, weight: isize) -> Self {
                    self.weight = weight;
                    self
                }

                pub(crate) fn run(&self, component: &mut $Component, cx: &mut Context) {
                    if let Some(action) = self.action {
                        action(component, cx)
                    }
                }
            }

            #[inline(always)]
            pub(crate) fn [<run_actions_after_prepare_ $Component:lower>](
                component: &mut $Component,
                cx: &mut Context
            ) {
                run_actions([<ACTION_AFTER_PREPARE_ $Component:upper>], |action|
                    action_ref::<[<AfterPrepare $Component>]>(&**action)
                        .run(component, cx)
                );
            }
        }
    };
}