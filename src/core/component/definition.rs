use crate::base::action;
use crate::core::component::Context;
use crate::core::AnyBase;
use crate::html::{html, Markup, PrepareMarkup};
use crate::{util, Weight};

pub trait ComponentBase {
    fn render(&mut self, cx: &mut Context) -> Markup;
}

pub trait ComponentTrait: AnyBase + ComponentBase + Send + Sync {
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
    fn setup_before_prepare(&mut self, cx: &mut Context) {}

    #[allow(unused_variables)]
    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::None
    }
}

impl<C: ComponentTrait> ComponentBase for C {
    fn render(&mut self, cx: &mut Context) -> Markup {
        if self.is_renderable(cx) {
            // Comprueba el componente antes de prepararlo.
            self.setup_before_prepare(cx);

            // Acciones del tema antes de preparar el componente.
            cx.theme().before_prepare_component(self, cx);

            // Acciones de los módulos antes de preparar el componente.
            action::component::BeforePrepareComponent::dispatch(self, cx, None);
            if let Some(id) = self.id() {
                action::component::BeforePrepareComponent::dispatch(self, cx, Some(id));
            }

            // Renderiza el componente.
            let markup = match cx.theme().render_component(self, cx) {
                Some(html) => html,
                None => match self.prepare_component(cx) {
                    PrepareMarkup::None => html! {},
                    PrepareMarkup::Text(text) => html! { (text) },
                    PrepareMarkup::With(html) => html,
                },
            };

            // Acciones del tema después de preparar el componente.
            cx.theme().after_prepare_component(self, cx);

            // Acciones de los módulos después de preparar el componente.
            action::component::AfterPrepareComponent::dispatch(self, cx, None);
            if let Some(id) = self.id() {
                action::component::AfterPrepareComponent::dispatch(self, cx, Some(id));
            }

            markup
        } else {
            html! {}
        }
    }
}

pub fn component_as_ref<C: ComponentTrait>(component: &dyn ComponentTrait) -> Option<&C> {
    component.as_any_ref().downcast_ref::<C>()
}

pub fn component_as_mut<C: ComponentTrait>(component: &mut dyn ComponentTrait) -> Option<&mut C> {
    component.as_any_mut().downcast_mut::<C>()
}
