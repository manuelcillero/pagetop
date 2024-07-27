use crate::prelude::*;

use std::fmt;

#[derive(AutoDefault)]
pub enum HeadingType {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(AutoDefault)]
pub enum HeadingSize {
    ExtraLarge,
    XxLarge,
    XLarge,
    Large,
    Medium,
    #[default]
    Normal,
    Subtitle,
}

#[rustfmt::skip]
impl fmt::Display for HeadingSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeadingSize::ExtraLarge => write!(f, "heading__title-x3l"),
            HeadingSize::XxLarge    => write!(f, "heading__title-x2l"),
            HeadingSize::XLarge     => write!(f, "heading__title-xl"),
            HeadingSize::Large      => write!(f, "heading__title-l"),
            HeadingSize::Medium     => write!(f, "heading__title-m"),
            HeadingSize::Normal     => write!(f, ""),
            HeadingSize::Subtitle   => write!(f, "heading__subtitle"),
        }
    }
}

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Heading {
    id          : OptionId,
    classes     : OptionClasses,
    heading_type: HeadingType,
    size        : HeadingSize,
    text        : OptionTranslated,
}

impl ComponentTrait for Heading {
    fn new() -> Self {
        Heading::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.set_classes(ClassesOp::Add, self.size().to_string());
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let id = self.id();
        let classes = self.classes().get();
        let text = self.text().escaped(cx.langid());
        PrepareMarkup::With(html! { @match &self.heading_type() {
            HeadingType::H1 => h1 id=[id] class=[classes] { (text) },
            HeadingType::H2 => h2 id=[id] class=[classes] { (text) },
            HeadingType::H3 => h3 id=[id] class=[classes] { (text) },
            HeadingType::H4 => h4 id=[id] class=[classes] { (text) },
            HeadingType::H5 => h5 id=[id] class=[classes] { (text) },
            HeadingType::H6 => h6 id=[id] class=[classes] { (text) },
        }})
    }
}

impl Heading {
    pub fn h1(text: L10n) -> Self {
        Heading::default()
            .with_heading_type(HeadingType::H1)
            .with_text(text)
    }

    pub fn h2(text: L10n) -> Self {
        Heading::default()
            .with_heading_type(HeadingType::H2)
            .with_text(text)
    }

    pub fn h3(text: L10n) -> Self {
        Heading::default()
            .with_heading_type(HeadingType::H3)
            .with_text(text)
    }

    pub fn h4(text: L10n) -> Self {
        Heading::default()
            .with_heading_type(HeadingType::H4)
            .with_text(text)
    }

    pub fn h5(text: L10n) -> Self {
        Heading::default()
            .with_heading_type(HeadingType::H5)
            .with_text(text)
    }

    pub fn h6(text: L10n) -> Self {
        Heading::default()
            .with_heading_type(HeadingType::H6)
            .with_text(text)
    }

    // Heading BUILDER.

    #[fn_builder]
    pub fn set_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.set_value(id);
        self
    }

    #[fn_builder]
    pub fn set_heading_type(&mut self, heading_type: HeadingType) -> &mut Self {
        self.heading_type = heading_type;
        self
    }

    #[fn_builder]
    pub fn set_size(&mut self, size: HeadingSize) -> &mut Self {
        self.size = size;
        self
    }

    #[fn_builder]
    pub fn set_text(&mut self, text: L10n) -> &mut Self {
        self.text.set_value(text);
        self
    }

    // Paragraph GETTERS.

    pub fn heading_type(&self) -> &HeadingType {
        &self.heading_type
    }

    pub fn size(&self) -> &HeadingSize {
        &self.size
    }

    pub fn text(&self) -> &OptionTranslated {
        &self.text
    }
}
