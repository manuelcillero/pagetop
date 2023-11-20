use crate::prelude::*;
use crate::BaseHandle;

#[derive(SmartDefault)]
pub enum HeadingType {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(SmartDefault)]
pub enum HeadingDisplay {
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
impl ToString for HeadingDisplay {
    fn to_string(&self) -> String {
        String::from(match self {
            HeadingDisplay::ExtraLarge => "pt-heading__title-x3l",
            HeadingDisplay::XxLarge    => "pt-heading__title-x2l",
            HeadingDisplay::XLarge     => "pt-heading__title-xl",
            HeadingDisplay::Large      => "pt-heading__title-l",
            HeadingDisplay::Medium     => "pt-heading__title-m",
            HeadingDisplay::Normal     => "",
            HeadingDisplay::Subtitle   => "pt-heading__subtitle",
        })
    }
}

#[rustfmt::skip]
#[derive(BaseHandle, ComponentClasses, SmartDefault)]
pub struct Heading {
    id          : OptionId,
    weight      : Weight,
    renderable  : Renderable,
    classes     : OptionClasses,
    heading_type: HeadingType,
    text        : OptionTranslated,
    display     : HeadingDisplay,
}

impl ComponentTrait for Heading {
    fn new() -> Self {
        Heading::default()
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

    #[rustfmt::skip]
    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.add_classes(self.display().to_string());
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
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

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
    pub fn alter_heading_type(&mut self, heading_type: HeadingType) -> &mut Self {
        self.heading_type = heading_type;
        self
    }

    #[fn_builder]
    pub fn alter_text(&mut self, text: L10n) -> &mut Self {
        self.text.alter_value(text);
        self
    }

    #[fn_builder]
    pub fn alter_display(&mut self, display: HeadingDisplay) -> &mut Self {
        self.display = display;
        self
    }

    // Paragraph GETTERS.

    pub fn heading_type(&self) -> &HeadingType {
        &self.heading_type
    }

    pub fn text(&self) -> &OptionTranslated {
        &self.text
    }

    pub fn display(&self) -> &HeadingDisplay {
        &self.display
    }
}
