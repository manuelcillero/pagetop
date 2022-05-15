use crate::util;
use crate::html::{Markup, html};
use crate::core::hook::{hook_ref, run_hooks};
use super::{BEFORE_RENDER_COMPONENT_HOOK, BeforeRenderComponentHook, InContext};

pub use std::any::Any as AnyComponent;

pub trait ComponentTrait: AnyComponent + Send + Sync {
    fn new() -> Self where Self: Sized;

    fn handler(&self) -> &'static str;

    fn name(&self) -> String {
        util::single_type_name::<Self>().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn is_renderable(&self) -> bool {
        true
    }

    fn weight(&self) -> isize {
        0
    }

    #[allow(unused_variables)]
    fn before_render(&mut self, context: &mut InContext) {
    }

    #[allow(unused_variables)]
    fn default_render(&self, context: &mut InContext) -> Markup {
        html! {}
    }

    fn as_ref_any(&self) -> &dyn AnyComponent;

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}

pub fn component_ref<C: 'static>(component: &dyn ComponentTrait) -> &C {
    component.as_ref_any().downcast_ref::<C>().unwrap()
}

pub fn component_mut<C: 'static>(component: &mut dyn ComponentTrait) -> &mut C {
    component.as_mut_any().downcast_mut::<C>().unwrap()
}

pub fn render_component(component: &mut dyn ComponentTrait, context: &mut InContext) -> Markup {
    // Acciones del componente antes de renderizar.
    component.before_render(context);

    // Acciones de los módulos antes de renderizar el componente.
    run_hooks(
        BEFORE_RENDER_COMPONENT_HOOK,
        |a| hook_ref::<BeforeRenderComponentHook>(&**a).run(component, context)
    );

    // Acciones del tema antes de renderizar el componente.
    context.theme().before_render_component(component, context);

    match component.is_renderable() {
        true => {
            match context.theme().render_component(component, context) {
                Some(html) => html,
                None => component.default_render(context)
            }
        },
        false => html! {}
    }
}
