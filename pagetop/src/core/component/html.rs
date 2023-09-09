use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup, PrepareMarkup};
use crate::{fn_builder, new_handle, Handle};

new_handle!(COMPONENT_HTML);

#[derive(Default)]
pub struct Html(Markup);

impl ComponentTrait for Html {
    fn new() -> Self {
        Html::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_HTML
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
