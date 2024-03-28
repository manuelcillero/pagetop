use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Body(MixedComponents);

impl ComponentTrait for Body {
    fn new() -> Self {
        Body::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let skip_to_id = cx.body_skip_to().get().unwrap_or("content".to_owned());

        PrepareMarkup::With(html! {
            body id=[cx.body_id().get()] class=[cx.body_classes().get()] {
                @if let Some(skip) = L10n::l("skip_to_content").using(cx.langid()) {
                    div class="skip__to_content" {
                        a href=(concat_string!("#", skip_to_id)) { (skip) }
                    }
                }
                (self.components().render(cx))
            }
        })
    }
}

impl Body {
    pub fn with(component: impl ComponentTrait) -> Self {
        Body::default().add_component(component)
    }

    // Body BUILDER.

    #[fn_builder]
    pub fn alter_components(&mut self, op: AnyOp) -> &mut Self {
        self.0.alter_value(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.0.alter_value(AnyOp::Add(AnyComponent::with(component)));
        self
    }

    // Body GETTERS.

    pub fn components(&self) -> &MixedComponents {
        &self.0
    }
}
