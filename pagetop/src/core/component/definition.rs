use crate::core::component::RenderContext;
use crate::html::{html, Markup};
use crate::{util, Handle};

pub use std::any::Any as AnyComponent;

pub trait BaseComponent {
    fn prepare(&mut self, rcx: &mut RenderContext) -> Markup;
}

pub trait ComponentTrait: AnyComponent + BaseComponent + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handle(&self) -> Handle;

    fn name(&self) -> String {
        util::single_type_name::<Self>().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn id(&self) -> Option<String> {
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
    fn before_prepare(&mut self, rcx: &mut RenderContext) {}

    #[allow(unused_variables)]
    fn prepare_component(&self, rcx: &mut RenderContext) -> Markup {
        html! {}
    }

    #[allow(unused_variables)]
    fn after_prepare(&mut self, rcx: &mut RenderContext) {}

    fn as_ref_any(&self) -> &dyn AnyComponent;

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}

impl<C: ComponentTrait> BaseComponent for C {
    fn prepare(&mut self, rcx: &mut RenderContext) -> Markup {
        if self.is_renderable(rcx) {
            // Acciones antes de preparar el componente.
            self.before_prepare(rcx);

            // Acciones del tema antes de preparar el componente.
            rcx.theme().before_prepare_component(self, rcx);

            let markup = match rcx.theme().render_component(self, rcx) {
                Some(html) => html,
                None => self.prepare_component(rcx),
            };

            // Acciones después de preparar el componente.
            self.after_prepare(rcx);

            // Acciones del tema después de preparar el componente.
            rcx.theme().after_prepare_component(self, rcx);

            markup
        } else {
            html! {}
        }
    }
}

pub fn component_ref<C: 'static>(component: &dyn ComponentTrait) -> &C {
    component.as_ref_any().downcast_ref::<C>().unwrap()
}

pub fn component_mut<C: 'static>(component: &mut dyn ComponentTrait) -> &mut C {
    component.as_mut_any().downcast_mut::<C>().unwrap()
}
