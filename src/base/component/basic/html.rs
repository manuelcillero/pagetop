use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Html(Markup);

impl ComponentTrait for Html {
    fn new() -> Self {
        Html::default()
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! { (self.0) })
    }
}

impl Html {
    pub fn with(html: Markup) -> Self {
        Html(html)
    }

    pub fn set_html(&mut self, html: Markup) -> &mut Self {
        self.0 = html;
        self
    }
}
