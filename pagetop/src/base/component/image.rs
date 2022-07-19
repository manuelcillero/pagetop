use crate::prelude::*;

pub const COMPONENT_IMAGE: &str = "pagetop::component::image";

pub struct Image {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    source    : AttributeValue,
    template  : String,
}

impl ComponentTrait for Image {
    fn new() -> Self {
        Image {
            weight    : 0,
            renderable: render_always,
            id        : IdentifierValue::new(),
            classes   : Classes::new_with_default("img-fluid"),
            source    : AttributeValue::new(),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        COMPONENT_IMAGE
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &InContext) -> bool {
        (self.renderable)(context)
    }

    fn default_render(&self, _: &mut InContext) -> Markup {
        html! {
            img
                src=[self.source().get()]
                id=[self.id().get()]
                class=[self.classes().get()];
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
    pub fn new_with_source(source: &str) -> Self {
        Image::new().with_source(source)
    }

    // Image BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
        self
    }

    pub fn with_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_classes(op, classes);
        self
    }

    pub fn with_source(mut self, source: &str) -> Self {
        self.alter_source(source);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Image ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, renderable: Renderable) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter(op, classes);
        self
    }

    pub fn alter_source(&mut self, source: &str) -> &mut Self {
        self.source.with_value(source);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Image GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn source(&self) -> &AttributeValue {
        &self.source
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
