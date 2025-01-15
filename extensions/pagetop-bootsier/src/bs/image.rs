use pagetop::prelude::*;

use crate::bs::Border;

use std::fmt;

#[derive(AutoDefault)]
pub enum ImageType {
    #[default]
    Fluid,
    Thumbnail,
}

#[rustfmt::skip]
impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageType::Fluid     => write!(f, "img-fluid"),
            ImageType::Thumbnail => write!(f, "img-thumbnail"),
        }
    }
}

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
#[derive(AutoDefault)]
pub struct Image {
    id        : OptionId,
    classes   : OptionClasses,
    image_type: ImageType,
    source    : OptionString,
    size      : ImageSize,
    border    : Border,
}

impl ComponentTrait for Image {
    fn new() -> Self {
        Image::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [self.image_type().to_string()].join(" "),
        );
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
        Image::default().with_source(source)
    }

    pub fn thumbnail(source: &str) -> Self {
        Image::default()
            .with_source(source)
            .with_image_type(ImageType::Thumbnail)
    }
    /*
        pub fn pagetop() -> Self {
            Image::default()
                .with_source("/base/pagetop-logo.svg")
                .with_classes(ClassesOp::Add, IMG_FIXED)
                .with_size(ImageSize::Size(64, 64))
        }
    */
    // Image BUILDER.

    #[fn_builder]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl Into<String>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn with_image_type(mut self, image_type: ImageType) -> Self {
        self.image_type = image_type;
        self
    }

    #[fn_builder]
    pub fn with_source(mut self, source: &str) -> Self {
        self.source.alter_value(source);
        self
    }

    #[fn_builder]
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = size;
        self
    }

    #[fn_builder]
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    // Image GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn image_type(&self) -> &ImageType {
        &self.image_type
    }

    pub fn source(&self) -> &OptionString {
        &self.source
    }

    pub fn size(&self) -> &ImageSize {
        &self.size
    }

    pub fn border(&self) -> &Border {
        &self.border
    }
}
