#[macro_export]
macro_rules! action_before_prepare_component {
    ( $ACTION_HANDLE:ident for $Component:ty ) => {
        $crate::paste! {
            $crate::use_handle!($ACTION_HANDLE);

            pub type ActionBefore = fn(component: &$Component, rcx: &mut RenderContext);

            pub struct [<BeforePrepare $Component>] {
                action: Option<ActionBefore>,
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
                    $ACTION_HANDLE
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
                pub fn with_action(mut self, action: ActionBefore) -> Self {
                    self.action = Some(action);
                    self
                }

                #[allow(dead_code)]
                pub fn with_weight(mut self, weight: isize) -> Self {
                    self.weight = weight;
                    self
                }

                pub fn run(&self, component: &mut $Component, rcx: &mut RenderContext) {
                    if let Some(action) = self.action {
                        action(component, rcx)
                    }
                }
            }

            #[inline(always)]
            pub fn run_actions_before_prepare_component(
                component: &mut $Component,
                rcx: &mut RenderContext
            ) {
                run_actions($ACTION_HANDLE, |action|
                    action_ref::<[<BeforePrepare $Component>]>(&**action)
                        .run(component, rcx)
                );
            }
        }
    };
}
