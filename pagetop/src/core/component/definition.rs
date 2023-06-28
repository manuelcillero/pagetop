use crate::core::component::Context;
use crate::html::{html, Markup, PrepareMarkup};
use crate::{util, Handle};

pub use std::any::Any as AnyComponent;

pub trait BaseComponent {
    fn prepare(&mut self, cx: &mut Context) -> Markup;
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
    fn is_renderable(&self, cx: &Context) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn before_prepare_component(&mut self, cx: &mut Context) {}

    #[allow(unused_variables)]
    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::None
    }

    #[allow(unused_variables)]
    fn after_prepare_component(&mut self, cx: &mut Context) {}

    fn as_ref_any(&self) -> &dyn AnyComponent;

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}

impl<C: ComponentTrait> BaseComponent for C {
    fn prepare(&mut self, cx: &mut Context) -> Markup {
        if self.is_renderable(cx) {
            // Acciones antes de preparar el componente.
            self.before_prepare_component(cx);

            // Acciones del tema antes de preparar el componente.
            cx.theme().before_prepare_component(self, cx);

            let markup = match cx.theme().render_component(self, cx) {
                Some(html) => html,
                None => self.prepare_component(cx).html(),
            };

            // Acciones después de preparar el componente.
            self.after_prepare_component(cx);

            // Acciones del tema después de preparar el componente.
            cx.theme().after_prepare_component(self, cx);

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
