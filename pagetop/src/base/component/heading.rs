use crate::prelude::*;

new_handle!(COMPONENT_BASE_HEADING);

#[derive(Default)]
pub enum HeadingType {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Default)]
pub enum HeadingDisplay {
    #[default]
    Normal,
    XxLarge,
    Large,
    Medium,
    Small,
    XxSmall,
    Subtitle,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Heading {
    weight      : Weight,
    renderable  : Renderable,
    id          : OptionId,
    classes     : OptionClasses,
    heading_type: HeadingType,
    text        : OptionTranslate,
    display     : HeadingDisplay,
    template    : String,
}

impl ComponentTrait for Heading {
    fn new() -> Self {
        Heading::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_HEADING
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

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let id = self.id();
        let classes = self.classes().get();
        PrepareMarkup::With(html! { @match &self.heading_type() {
            HeadingType::H1 => h1 id=[id] class=[classes] { (self.text().escaped(cx.langid())) },
            HeadingType::H2 => h2 id=[id] class=[classes] { (self.text().escaped(cx.langid())) },
            HeadingType::H3 => h3 id=[id] class=[classes] { (self.text().escaped(cx.langid())) },
            HeadingType::H4 => h4 id=[id] class=[classes] { (self.text().escaped(cx.langid())) },
            HeadingType::H5 => h5 id=[id] class=[classes] { (self.text().escaped(cx.langid())) },
            HeadingType::H6 => h6 id=[id] class=[classes] { (self.text().escaped(cx.langid())) },
        }})
    }
}

impl Heading {
    pub fn h1(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H1)
            .with_text(text)
    }

    pub fn h2(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H2)
            .with_text(text)
    }

    pub fn h3(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H3)
            .with_text(text)
    }

    pub fn h4(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H4)
            .with_text(text)
    }

    pub fn h5(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H5)
            .with_text(text)
    }

    pub fn h6(text: L10n) -> Self {
        Heading::new()
            .with_heading_type(HeadingType::H6)
            .with_text(text)
    }

    // Heading BUILDER.

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
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
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

    #[rustfmt::skip]
    #[fn_builder]
    pub fn alter_display(&mut self, display: HeadingDisplay) -> &mut Self {
        self.alter_classes(
            ClassesOp::SetDefault,
            match display {
                HeadingDisplay::XxLarge  => "display-2",
                HeadingDisplay::Large    => "display-3",
                HeadingDisplay::Medium   => "display-4",
                HeadingDisplay::Small    => "display-5",
                HeadingDisplay::XxSmall  => "display-6",
                HeadingDisplay::Normal   => "",
                HeadingDisplay::Subtitle => "",
            },
        );
        self.display = display;
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Paragraph GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn heading_type(&self) -> &HeadingType {
        &self.heading_type
    }

    pub fn text(&self) -> &OptionTranslate {
        &self.text
    }

    pub fn display(&self) -> &HeadingDisplay {
        &self.display
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
