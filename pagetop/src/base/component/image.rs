use crate::prelude::*;

pub const TYPENAME_IMAGE: &str = "pagetop::base::component::image::Image";

pub struct Image {
    renderable: fn() -> bool,
    weight    : i8,
    source    : OptAttr,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl ComponentTrait for Image {
    fn new() -> Self {
        Image {
            renderable: render_always,
            weight    : 0,
            source    : OptAttr::new(),
            id        : OptIden::new(),
            classes   : Classes::new_with_default("img-fluid"),
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
                class=[self.classes()];
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Image {
    pub fn image(source: &str) -> Self {
        Image::new().with_source(source)
    }

    // Image BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_source(mut self, source: &str) -> Self {
        self.alter_source(source);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
        self
    }

    pub fn with_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_classes(classes, op);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Image ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: i8) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_source(&mut self, source: &str) -> &mut Self {
        self.source.with_value(source);
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
        self
    }

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
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

    pub fn classes(&self) -> &Option<String> {
        self.classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
