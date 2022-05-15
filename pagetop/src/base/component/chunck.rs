use crate::prelude::*;

pub const CHUNCK_COMPONENT: &str = "pagetop::component::chunck";

pub struct Chunck {
    renderable: fn() -> bool,
    weight    : isize,
    html      : Markup,
    template  : String,
}

impl ComponentTrait for Chunck {
    fn new() -> Self {
        Chunck {
            renderable: render_always,
            weight    : 0,
            html      : html! {},
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        CHUNCK_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, _: &mut Context) -> Markup {
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

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
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

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
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
