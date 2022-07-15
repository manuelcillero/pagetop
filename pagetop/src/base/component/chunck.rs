use crate::prelude::*;

pub const COMPONENT_CHUNCK: &str = "pagetop::component::chunck";

pub struct Chunck {
    weight    : isize,
    renderable: Renderable,
    html      : Markup,
    template  : String,
}

impl ComponentTrait for Chunck {
    fn new() -> Self {
        Chunck {
            weight    : 0,
            renderable: render_always,
            html      : html! {},
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        COMPONENT_CHUNCK
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &InContext) -> bool {
        (self.renderable)(context)
    }

    fn default_render(&self, _: &mut InContext) -> Markup {
        html! { (*self.html()) }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Chunck {
    pub fn with(html: Markup) -> Self {
        Chunck::new().with_html(html)
    }

    // Chunck BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_html(mut self, html: Markup) -> Self {
        self.alter_html(html);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Chunck ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, renderable: Renderable) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.html = html;
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Chunck GETTERS.

    pub fn html(&self) -> &Markup {
        &self.html
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
