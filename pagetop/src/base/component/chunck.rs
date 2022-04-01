use crate::prelude::*;

pub struct Chunck {
    renderable: fn() -> bool,
    weight    : i8,
    html      : Vec<Markup>,
    template  : String,
}

impl PageComponent for Chunck {
    fn new() -> Self {
        Chunck {
            renderable: always,
            weight    : 0,
            html      : Vec::new(),
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
        html! {
            @for html in self.html().iter() {
                (*html)
            }
        }
    }
}

impl Chunck {
    pub fn with(html: Markup) -> Self {
        Chunck::new().add(html)
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

    pub fn add(mut self, html: Markup) -> Self {
        self.html.push(html);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Chunck GETTERS.

    pub fn html(&self) -> &Vec<Markup> {
        &self.html
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
