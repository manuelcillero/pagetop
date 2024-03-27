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
            action::theme::BeforePrepare::dispatch(self, cx);

            // Acciones de los módulos antes de preparar el componente.
            action::component::BeforePrepare::dispatch(self, cx);
            action::component::BeforePrepare::dispatch_by_id(self, cx);

            // Renderiza el componente.
            let markup = match action::theme::RenderComponent::dispatch(self, cx) {
                Some(html) => html,
                None => match self.prepare_component(cx) {
                    PrepareMarkup::None => html! {},
                    PrepareMarkup::Text(text) => html! { (text) },
                    PrepareMarkup::With(html) => html,
                },
            };

            // Acciones del tema después de preparar el componente.
            action::theme::AfterPrepare::dispatch(self, cx);

            // Acciones de los módulos después de preparar el componente.
            action::component::AfterPrepare::dispatch(self, cx);
            action::component::AfterPrepare::dispatch_by_id(self, cx);

            markup
        } else {
            html! {}
        }
    }
}
