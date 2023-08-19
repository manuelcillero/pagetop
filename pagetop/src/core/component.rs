mod context;
pub use context::{Context, ContextOp};
pub type FnContextualPath = fn(cx: &Context) -> &str;

mod renderable;
pub use renderable::{FnIsRenderable, Renderable};

mod definition;
pub use definition::{component_as_mut, component_as_ref, ComponentBase, ComponentTrait};

mod arc;
pub use arc::{ArcComponent, ArcComponents, ArcOp};

mod typed;
pub use typed::{TypedComponent, TypedComponents, TypedOp};

pub mod html;
pub mod l10n;

#[macro_export]
macro_rules! actions_for_component {
    ( $Component:ty ) => {
        $crate::paste! {
            use $crate::prelude::*;

            pub type [<FnAction $Component>] = fn(component: &$Component, cx: &mut Context);

            // *************************************************************************************
            // ACTION BEFORE PREPARE COMPONENT
            // *************************************************************************************

            $crate::new_handle!([<ACTION_BEFORE_PREPARE_ $Component:upper>] for Crate);

            pub struct [<BeforePrepare $Component>] {
                action: Option<[<FnAction $Component>]>,
                weight: Weight,
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

                fn weight(&self) -> Weight {
                    self.weight
                }
            }

            impl [<BeforePrepare $Component>] {
                #[allow(dead_code)]
                pub fn with_action(mut self, action: [<FnAction $Component>]) -> Self {
                    self.action = Some(action);
                    self
                }

                #[allow(dead_code)]
                pub fn with_weight(mut self, value: Weight) -> Self {
                    self.weight = value;
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

            $crate::new_handle!([<ACTION_AFTER_PREPARE_ $Component:upper>] for Crate);

            pub struct [<AfterPrepare $Component>] {
                action: Option<[<FnAction $Component>]>,
                weight: Weight,
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

                fn weight(&self) -> Weight {
                    self.weight
                }
            }

            impl [<AfterPrepare $Component>] {
                #[allow(dead_code)]
                pub fn with_action(mut self, action: [<FnAction $Component>]) -> Self {
                    self.action = Some(action);
                    self
                }

                #[allow(dead_code)]
                pub fn with_weight(mut self, value: Weight) -> Self {
                    self.weight = value;
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
