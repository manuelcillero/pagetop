use crate::prelude::*;

pub_handle!(COMPONENT_IMAGE);

#[rustfmt::skip]
#[derive(Default)]
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
        Image::default().with_classes(ClassesOp::SetDefault, "img-fluid")
    }

    fn handle(&self) -> Handle {
        COMPONENT_IMAGE
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn default_render(&self, _: &mut RenderContext) -> Markup {
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
    pub fn with(source: &str) -> Self {
        Image::new().with_source(source)
    }

    // Image BUILDER.

    #[fn_with]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    #[fn_with]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_with]
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_with]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_with]
    pub fn alter_source(&mut self, source: &str) -> &mut Self {
        self.source.alter_value(source);
        self
    }

    #[fn_with]
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
