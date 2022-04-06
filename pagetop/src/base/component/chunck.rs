use crate::prelude::*;

pub struct Chunck {
    renderable: fn() -> bool,
    weight    : i8,
    html      : Markup,
    template  : String,
}

impl PageComponent for Chunck {
    fn new() -> Self {
        Chunck {
            renderable: always,
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
}

impl Chunck {
    pub fn with(html: Markup) -> Self {
        let mut chunck = Chunck::new();
        chunck.html = html;
        chunck
    }

    // Chunck BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
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

fn always() -> bool {
    true
}
