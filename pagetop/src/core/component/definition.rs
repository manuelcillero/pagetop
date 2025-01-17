use crate::base::action;
use crate::core::component::Context;
use crate::core::AnyBase;
use crate::html::{html, Markup, PrepareMarkup};
use crate::util::TypeInfo;

pub trait ComponentBase {
    fn render(&mut self, cx: &mut Context) -> Markup;
}

pub trait ComponentTrait: AnyBase + ComponentBase + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn name(&self) -> &'static str {
        TypeInfo::ShortName.of::<Self>()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn id(&self) -> Option<String> {
        None
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
        if action::component::IsRenderable::dispatch(self, cx) {
            // Configura el componente antes de preparar.
            self.setup_before_prepare(cx);

            // Acciones específicas del tema antes de renderizar el componente.
            action::theme::BeforeRender::dispatch(self, cx);

            // Acciones de las extensiones antes de renderizar el componente.
            action::component::BeforeRender::dispatch(self, cx);

            // Renderiza el componente.
            let markup = match action::theme::RenderComponent::dispatch(self, cx) {
                Some(html) => html,
                None => self.prepare_component(cx).render(),
            };

            // Acciones específicas del tema después de renderizar el componente.
            action::theme::AfterRender::dispatch(self, cx);

            // Acciones de las extensiones después de renderizar el componente.
            action::component::AfterRender::dispatch(self, cx);

            markup
        } else {
            html! {}
        }
    }
}
