use crate::core::component::RenderContext;
use crate::html::{html, Markup};
use crate::{define_handle, util, Handle};

pub use std::any::Any as AnyComponent;

define_handle!(COMPONENT_UNNAMED);

pub trait BaseComponent {
    fn render(&mut self, rcx: &mut RenderContext) -> Markup;
}

pub trait ComponentTrait: AnyComponent + BaseComponent + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handle(&self) -> Handle {
        COMPONENT_UNNAMED
    }

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
