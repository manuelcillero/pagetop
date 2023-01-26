use crate::prelude::*;

pub_handle!(COMPONENT_HTML);

#[rustfmt::skip]
#[derive(Default)]
pub struct Html {
    weight    : isize,
    renderable: Renderable,
    html      : HtmlMarkup,
    template  : String,
}

impl ComponentTrait for Html {
    fn new() -> Self {
        Html::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_HTML
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn default_render(&self, _: &mut RenderContext) -> Markup {
        html! { (*self.html()) }
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
        Html::new().with_html(html)
    }

    // Html BUILDER.

    #[fn_with]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    #[fn_with]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_with]
    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.html.markup = html;
        self
    }

    #[fn_with]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Html GETTERS.

    pub fn html(&self) -> &Markup {
        &self.html.markup
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
