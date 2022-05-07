use crate::html::{Markup, html};
use crate::api::{TypeId, action::{action_ref, run_actions}};
use crate::util;
use super::{ActionBeforeRenderComponent, Assets};

pub use std::any::Any as AnyComponent;

pub trait BaseComponent {
    fn type_name(&self) -> &'static str;

    fn single_name(&self) -> &'static str;

    fn qualified_name(&self, last: usize) -> &'static str;
}

pub trait ComponentTrait: AnyComponent + BaseComponent + Send + Sync {

    fn new() -> Self where Self: Sized;

    fn name(&self) -> String {
        self.single_name().to_owned()
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

impl<C: ?Sized + ComponentTrait> BaseComponent for C {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn single_name(&self) -> &'static str {
        util::partial_type_name(std::any::type_name::<Self>(), 1)
    }

    fn qualified_name(&self, last: usize) -> &'static str {
        util::partial_type_name(std::any::type_name::<Self>(), last)
    }
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
        TypeId::of::<ActionBeforeRenderComponent>(),
        |a| action_ref::<ActionBeforeRenderComponent>(&**a).run(component, assets)
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
