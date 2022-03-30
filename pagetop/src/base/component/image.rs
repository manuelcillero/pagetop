use crate::prelude::*;

pub struct Image {
    renderable: fn() -> bool,
    weight    : i8,
    source    : OptAttr,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl PageComponent for Image {

    fn new() -> Self {
        Image {
            renderable: always,
            weight    : 0,
            source    : OptAttr::none(),
            id        : OptIden::none(),
            classes   : Classes::none(),
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
            img
                src=[self.source()]
                id=[self.id()]
                class=[self.classes("img-fluid")];
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

    pub fn with_id(mut self, id: &str) -> Self {
        self.id.with_value(id);
        self
    }

    pub fn set_classes(mut self, classes: &str) -> Self {
        self.classes.set_classes(classes);
        self
    }

    pub fn add_classes(mut self, classes: &str) -> Self {
        self.classes.add_classes(classes);
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

    pub fn id(&self) -> &Option<String> {
        self.id.option()
    }

    pub fn classes(&self, default: &str) -> Option<String> {
        self.classes.option(default)
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
