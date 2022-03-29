use crate::prelude::*;

pub struct Image {
    renderable: fn() -> bool,
    weight    : i8,
    source    : OptAttr,
    template  : String,
}

impl PageComponent for Image {

    fn new() -> Self {
        Image {
            renderable: always,
            weight    : 0,
            source    : OptAttr::none(),
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
            img src=[self.source()] class="img-fluid";
        }
    }
}

impl Image {

    pub fn image(source: &str) -> Self {
        Image::new().with_source(source)
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

    pub fn with_source(mut self, source: &str) -> Self {
        self.source.with_value(source);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Image GETTERS.

    pub fn source(&self) -> &Option<String> {
        self.source.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
