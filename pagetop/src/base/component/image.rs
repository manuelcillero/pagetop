use crate::prelude::*;

pub struct Image {
    renderable: fn() -> bool,
    weight    : i8,
    source    : Option<String>,
    template  : String,
}

impl PageComponent for Image {

    fn new() -> Self {
        Image {
            renderable: always,
            weight    : 0,
            source    : None,
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
            img src=[&self.source] class="img-fluid" {}
        }
    }
}

impl Image {

    pub fn image(source: &str) -> Self {
        let mut i = Image::new();
        i.source = Some(source.to_owned());
        i
    }

    // Image BUILDER.

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

    // Image GETTERS.

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
