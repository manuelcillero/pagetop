use crate::prelude::*;

const IMG_FLUID: &str = "pt-img__fluid";
const IMG_FIXED: &str = "pt-img__fixed";

#[derive(AutoDefault)]
pub enum ImageSize {
    #[default]
    Auto,
    Size(u16, u16),
    Width(u16),
    Height(u16),
    Both(u16),
}

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Image {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    classes   : OptionClasses,
    source    : OptionString,
    size      : ImageSize,
}

impl ComponentTrait for Image {
    fn new() -> Self {
        Image::default().with_classes(ClassesOp::Add, IMG_FLUID)
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
            ImageSize::Both(value) => (Some(value), Some(value)),
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
        Image::default()
            .with_source(source)
            .with_classes(ClassesOp::Add, IMG_FLUID)
    }

    pub fn fixed(source: &str) -> Self {
        Image::default()
            .with_source(source)
            .with_classes(ClassesOp::Add, IMG_FIXED)
    }

    pub fn pagetop() -> Self {
        Image::default()
            .with_source("/base/pagetop-logo.svg")
            .with_classes(ClassesOp::Add, IMG_FIXED)
            .with_size(ImageSize::Size(64, 64))
    }

    // Image BUILDER.

    #[fn_with]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_with]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_with]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_with]
    pub fn alter_source(&mut self, source: &str) -> &mut Self {
        self.source.alter_value(source);
        self
    }

    #[fn_with]
    pub fn alter_size(&mut self, size: ImageSize) -> &mut Self {
        self.size = size;
        self
    }

    // Image GETTERS.

    pub fn source(&self) -> &OptionString {
        &self.source
    }

    pub fn size(&self) -> &ImageSize {
        &self.size
    }
}
