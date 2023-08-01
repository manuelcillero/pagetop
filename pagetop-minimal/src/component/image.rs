use pagetop::prelude::*;

new_handle!(COMPONENT_IMAGE);

#[rustfmt::skip]
#[derive(Default)]
pub struct Image {
    weight    : Weight,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    source    : AttributeValue,
}

impl ComponentTrait for Image {
    fn new() -> Self {
        Image::default().with_classes(ClassesOp::SetDefault, "img-fluid")
    }

    fn handle(&self) -> Handle {
        COMPONENT_IMAGE
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, _: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            img
                src=[self.source().get()]
                id=[self.id()]
                class=[self.classes().get()];
        })
    }
}

impl Image {
    pub fn with(source: &str) -> Self {
        Image::new().with_source(source)
    }

    pub fn pagetop() -> Self {
        Image::new().with_source("/minimal/pagetop-logo.svg")
    }

    // Image BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_source(&mut self, source: &str) -> &mut Self {
        self.source.alter_value(source);
        self
    }

    // Image GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn source(&self) -> &AttributeValue {
        &self.source
    }
}
