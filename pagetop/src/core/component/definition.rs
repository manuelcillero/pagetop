use super::RenderResources;

use crate::html::{html, Markup};
use crate::util::{single_type_name, Handle};

pub use std::any::Any as AnyComponent;

pub trait BaseComponent {
    fn render(&mut self, rsx: &mut RenderResources) -> Markup;
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
    fn is_renderable(&self, rsx: &RenderResources) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn before_render(&mut self, rsx: &mut RenderResources) {}

    #[allow(unused_variables)]
    fn default_render(&self, rsx: &mut RenderResources) -> Markup {
        html! {}
    }

    fn as_ref_any(&self) -> &dyn AnyComponent;

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}

impl<C: ComponentTrait> BaseComponent for C {
    fn render(&mut self, rsx: &mut RenderResources) -> Markup {
        // Acciones del componente antes de renderizar.
        self.before_render(rsx);

        // Acciones del tema antes de renderizar el componente.
        rsx.theme().before_render_component(self, rsx);

        match self.is_renderable(rsx) {
            true => match rsx.theme().render_component(self, rsx) {
                Some(html) => html,
                None => self.default_render(rsx),
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
        paste::paste! {
            $crate::pub_handle!($ACTION_HANDLE);

            type Action = fn(&$Component, &mut RenderResources);

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

                pub fn run(&self, component: &mut $Component, rsx: &mut RenderResources) {
                    if let Some(action) = self.action {
                        action(component, rsx)
                    }
                }
            }

            #[inline(always)]
            fn before_render_inline(component: &mut $Component, rsx: &mut RenderResources) {
                run_actions($ACTION_HANDLE, |action|
                    action_ref::<[< BeforeRender $Component >]>(&**action)
                        .run(component, rsx)
                );
            }
        }
    };
}
