use crate::html::{Markup, html};
use crate::api::action::{action_ref, run_actions};
use crate::util;
use super::{BEFORE_RENDER_COMPONENT_ACTION, BeforeRenderComponentAction};
use super::Assets;

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
    fn before_render(&mut self, assets: &mut Assets) {
    }

    #[allow(unused_variables)]
    fn default_render(&self, assets: &mut Assets) -> Markup {
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

pub fn render_component(component: &mut dyn ComponentTrait, assets: &mut Assets) -> Markup {
    // Acciones del componente antes de renderizar.
    component.before_render(assets);

    // Acciones de los módulos antes de renderizar el componente.
    run_actions(
        BEFORE_RENDER_COMPONENT_ACTION,
        |a| action_ref::<BeforeRenderComponentAction>(&**a).run(component, assets)
    );

    // Acciones del tema antes de renderizar el componente.
    assets.theme().before_render_component(component, assets);

    match component.is_renderable() {
        true => {
            match assets.theme().render_component(component, assets) {
                Some(html) => html,
                None => component.default_render(assets)
            }
        },
        false => html! {}
    }
}
