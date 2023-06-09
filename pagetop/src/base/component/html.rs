use crate::prelude::*;

define_handle!(COMPONENT_HTML);

#[derive(Default)]
pub struct Html(Markup);

impl ComponentTrait for Html {
    fn new() -> Self {
        Html::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_HTML
    }

    fn default_render(&self, _: &mut RenderContext) -> Markup {
        html! { (self.html()) }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
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
