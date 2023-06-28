#[macro_export]
macro_rules! action_after_prepare_component {
    ( $ACTION_HANDLE:ident for $Component:ty ) => {
        $crate::paste! {
            $crate::use_handle!($ACTION_HANDLE);

            pub type ActionAfter = fn(component: &$Component, cx: &mut Context);

            pub struct [<AfterPrepare $Component>] {
                action: Option<ActionAfter>,
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
                    $ACTION_HANDLE
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
                pub fn with_action(mut self, action: ActionAfter) -> Self {
                    self.action = Some(action);
                    self
                }

                #[allow(dead_code)]
                pub fn with_weight(mut self, weight: isize) -> Self {
                    self.weight = weight;
                    self
                }

                pub fn run(&self, component: &mut $Component, cx: &mut Context) {
                    if let Some(action) = self.action {
                        action(component, cx)
                    }
                }
            }

            #[inline(always)]
            pub fn run_actions_after_prepare_component(
                component: &mut $Component,
                cx: &mut Context
            ) {
                run_actions($ACTION_HANDLE, |action|
                    action_ref::<[<AfterPrepare $Component>]>(&**action)
                        .run(component, cx)
                );
            }
        }
    };
}
