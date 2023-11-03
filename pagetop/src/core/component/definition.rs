use crate::base::action::component::run_actions_after_prepare_component;
use crate::base::action::component::run_actions_before_prepare_component;
use crate::core::component::Context;
use crate::html::{html, Markup, PrepareMarkup};
use crate::{util, HasHandle, Weight};

use std::any::Any;

pub trait ComponentBase: Any {
    fn render(&mut self, cx: &mut Context) -> Markup;

    fn as_ref_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub trait ComponentTrait: ComponentBase + HasHandle + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn name(&self) -> String {
        util::single_type_name::<Self>().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn id(&self) -> Option<String> {
        None
    }

    fn weight(&self) -> Weight {
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
}

impl<C: ComponentTrait> ComponentBase for C {
    fn render(&mut self, cx: &mut Context) -> Markup {
        if self.is_renderable(cx) {
            // Acciones antes de preparar el componente.
            self.before_prepare_component(cx);

            // Acciones del tema antes de preparar el componente.
            cx.theme().before_prepare_component(self, cx);

            // Acciones de los módulos antes de preparar el componente.
            run_actions_before_prepare_component(self, cx);

            let markup = match cx.theme().render_component(self, cx) {
                Some(html) => html,
                None => match self.prepare_component(cx) {
                    PrepareMarkup::None => html! {},
                    PrepareMarkup::Text(text) => html! { (text) },
                    PrepareMarkup::With(html) => html,
                },
            };

            // Acciones después de preparar el componente.
            self.after_prepare_component(cx);

            // Acciones del tema después de preparar el componente.
            cx.theme().after_prepare_component(self, cx);

            // Acciones de los módulos después de preparar el componente.
            run_actions_after_prepare_component(self, cx);

            markup
        } else {
            html! {}
        }
    }

    fn as_ref_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub fn component_as_ref<C: ComponentTrait>(component: &dyn ComponentTrait) -> &C {
    component.as_ref_any().downcast_ref::<C>().unwrap()
}

pub fn component_as_mut<C: ComponentTrait>(component: &mut dyn ComponentTrait) -> &mut C {
    component.as_mut_any().downcast_mut::<C>().unwrap()
}
