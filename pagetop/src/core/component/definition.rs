use super::InContext;
use crate::html::{html, Markup};
use crate::util;

pub use std::any::Any as AnyComponent;

pub trait BaseComponent {
    fn render(&mut self, context: &mut InContext) -> Markup;
}

pub trait ComponentTrait: AnyComponent + BaseComponent + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handler(&self) -> &'static str;

    fn name(&self) -> String {
        util::single_type_name::<Self>().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn weight(&self) -> isize {
        0
    }

    #[allow(unused_variables)]
    fn is_renderable(&self, context: &InContext) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn before_render(&mut self, context: &mut InContext) {}

    #[allow(unused_variables)]
    fn default_render(&self, context: &mut InContext) -> Markup {
        html! {}
    }

    fn as_ref_any(&self) -> &dyn AnyComponent;

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}

impl<C: ComponentTrait> BaseComponent for C {
    fn render(&mut self, context: &mut InContext) -> Markup {
        // Acciones del componente antes de renderizar.
        self.before_render(context);

        // Acciones del tema antes de renderizar el componente.
        context.theme().before_render_component(self, context);

        match self.is_renderable(context) {
            true => match context.theme().render_component(self, context) {
                Some(html) => html,
                None => self.default_render(context),
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
    ( $ACTION_HANDLER:ident = $handler:literal, $Component:ty ) => {
        paste::paste! {
            const $ACTION_HANDLER: &str = $handler;

            type Action = fn(&$Component, &mut InContext);

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

                fn handler(&self) -> &'static str {
                    $ACTION_HANDLER
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

                pub fn run(&self, component: &mut $Component, context: &mut InContext) {
                    if let Some(action) = self.action {
                        action(component, context)
                    }
                }
            }

            #[inline(always)]
            fn before_render_inline(component: &mut $Component, context: &mut InContext) {
                run_actions(
                    $ACTION_HANDLER,
                    |action| action_ref::<[< BeforeRender $Component >]>(&**action).run(component, context)
                );
            }
        }
    };
}
