use super::RenderContext;

use crate::html::{html, Markup};
use crate::util::single_type_name;
use crate::Handle;

pub use std::any::Any as AnyComponent;

pub trait BaseComponent {
    fn render(&mut self, rcx: &mut RenderContext) -> Markup;
}

pub trait ComponentTrait: AnyComponent + BaseComponent + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handle(&self) -> Handle;

    fn name(&self) -> String {
        single_type_name::<Self>().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn weight(&self) -> isize {
        0
    }

    #[allow(unused_variables)]
    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn before_render(&mut self, rcx: &mut RenderContext) {}

    #[allow(unused_variables)]
    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        html! {}
    }

    fn as_ref_any(&self) -> &dyn AnyComponent;

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}

impl<C: ComponentTrait> BaseComponent for C {
    fn render(&mut self, rcx: &mut RenderContext) -> Markup {
        // Acciones del componente antes de renderizar.
        self.before_render(rcx);

        // Acciones del tema antes de renderizar el componente.
        rcx.theme().before_render_component(self, rcx);

        match self.is_renderable(rcx) {
            true => match rcx.theme().render_component(self, rcx) {
                Some(html) => html,
                None => self.default_render(rcx),
            },
            false => html! {},
        }
    }
}

pub fn component_ref<C: 'static>(component: &dyn ComponentTrait) -> &C {
    component.as_ref_any().downcast_ref::<C>().unwrap()
}

pub fn component_mut<C: 'static>(component: &mut dyn ComponentTrait) -> &mut C {
    component.as_mut_any().downcast_mut::<C>().unwrap()
}

#[macro_export]
macro_rules! hook_before_render_component {
    ( $ACTION_HANDLE:ident, $Component:ty ) => {
        $crate::paste! {
            $crate::define_handle!($ACTION_HANDLE);

            type Action = fn(&$Component, &mut RenderContext);

            pub struct [< BeforeRender $Component >] {
                action: Option<Action>,
                weight: isize,
            }

            impl HookActionTrait for [< BeforeRender $Component >] {
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

                fn as_ref_any(&self) -> &dyn AnyHookAction {
                    self
                }
            }

            impl [< BeforeRender $Component >] {
                #[allow(dead_code)]
                pub fn with_hook(mut self, action: Action) -> Self {
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
