use crate::prelude::*;

pub const TYPENAME_CHUNCK: &str = "pagetop::base::component::chunck::Chunck";

pub struct Chunck {
    renderable: fn() -> bool,
    weight    : i8,
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

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, _: &mut PageAssets) -> Markup {
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

    pub fn with_weight(mut self, weight: i8) -> Self {
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

    pub fn alter_weight(&mut self, weight: i8) -> &mut Self {
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
