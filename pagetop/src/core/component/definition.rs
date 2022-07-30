use crate::html::{html, Markup};
use crate::response::page::PageContext;
use crate::util::{single_type_name, Handler};

pub use std::any::Any as AnyComponent;

pub trait BaseComponent {
    fn render(&mut self, context: &mut PageContext) -> Markup;
}

pub trait ComponentTrait: AnyComponent + BaseComponent + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handler(&self) -> Handler;

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
    fn is_renderable(&self, context: &PageContext) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn before_render(&mut self, context: &mut PageContext) {}

    #[allow(unused_variables)]
    fn default_render(&self, context: &mut PageContext) -> Markup {
        html! {}
    }

    fn as_ref_any(&self) -> &dyn AnyComponent;

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}

impl<C: ComponentTrait> BaseComponent for C {
    fn render(&mut self, context: &mut PageContext) -> Markup {
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
    ( $ACTION_HANDLER:ident, $Component:ty ) => {
        paste::paste! {
            $crate::pub_const_handler!($ACTION_HANDLER);

            type Action = fn(&$Component, &mut PageContext);

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

                fn handler(&self) -> Handler {
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

                pub fn run(&self, component: &mut $Component, context: &mut PageContext) {
                    if let Some(action) = self.action {
                        action(component, context)
                    }
                }
            }

            #[inline(always)]
            fn before_render_inline(component: &mut $Component, context: &mut PageContext) {
                run_actions(
                    $ACTION_HANDLER,
                    |action| action_ref::<[< BeforeRender $Component >]>(&**action).run(component, context)
                );
            }
        }
    };
}
