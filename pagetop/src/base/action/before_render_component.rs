#[macro_export]
macro_rules! action_before_render_component {
    ( $ACTION_HANDLE:ident for $Component:ty ) => {
        $crate::paste! {
            $crate::define_handle!($ACTION_HANDLE);

            type Action = fn(&$Component, &mut RenderContext);

            pub struct [< BeforeRender $Component >] {
                action: Option<Action>,
                weight: isize,
            }

            impl ActionTrait for [< BeforeRender $Component >] {
                fn new() -> Self {
                    [< BeforeRender $Component >] {
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

            impl [< BeforeRender $Component >] {
                #[allow(dead_code)]
                pub fn with_action(mut self, action: Action) -> Self {
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
            pub fn before_render_inline(component: &mut $Component, rcx: &mut RenderContext) {
                run_actions($ACTION_HANDLE, |action|
                    action_ref::<[< BeforeRender $Component >]>(&**action)
                        .run(component, rcx)
                );
            }
        }
    };
}
