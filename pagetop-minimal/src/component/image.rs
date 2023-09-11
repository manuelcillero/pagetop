use pagetop::prelude::*;

new_handle!(COMPONENT_IMAGE);

#[derive(Default)]
pub enum ImageSize {
    #[default]
    Auto,
    Size(u16, u16),
    Width(u16),
    Height(u16),
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Image {
    weight    : Weight,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    source    : AttributeValue,
    size      : ImageSize,
}

impl ComponentTrait for Image {
    fn new() -> Self {
        Image::default().with_classes(ClassesOp::SetDefault, &["img-fluid"])
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

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        let (width, height) = match self.size() {
            ImageSize::Auto => (None, None),
            ImageSize::Size(width, height) => (Some(width), Some(height)),
            ImageSize::Width(width) => (Some(width), None),
            ImageSize::Height(height) => (None, Some(height)),
        };
        PrepareMarkup::With(html! {
            img
                src=[self.source().get()]
                id=[self.id()]
                class=[self.classes().get()]
                width=[width]
                height=[height] {}
        })
    }
}

impl Image {
    pub fn with(source: &str) -> Self {
        Image::new().with_source(source)
    }

    pub fn pagetop() -> Self {
        Image::new()
            .with_source("/minimal/pagetop-logo.svg")
            .with_size(ImageSize::Size(64, 64))
    }

    // Image BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &[impl ToString]) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_source(&mut self, source: &str) -> &mut Self {
        self.source.alter_value(source);
        self
    }

    #[fn_builder]
    pub fn alter_size(&mut self, size: ImageSize) -> &mut Self {
        self.size = size;
        self
    }

    // Image GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn source(&self) -> &AttributeValue {
        &self.source
    }

    pub fn size(&self) -> &ImageSize {
        &self.size
    }
}
