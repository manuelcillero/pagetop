use crate::prelude::*;

#[derive(SmartDefault)]
pub struct Html(Markup);

impl_handle!(COMPONENT_BASE_HTML for Html);

impl ComponentTrait for Html {
    fn new() -> Self {
        Html::default()
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! { (self.html()) })
    }
}

impl Html {
    pub fn with(html: Markup) -> Self {
        Html(html)
    }

    // Html BUILDER.

    #[fn_builder]
    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.0 = html;
        self
    }

    // Html GETTERS.

    pub fn html(&self) -> &Markup {
        &self.0
    }
}
