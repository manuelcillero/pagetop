use crate::prelude::*;

pub const HEADING_COMPONENT: &str = "pagetop::component::heading";

pub enum HeadingType { H1, H2, H3, H4, H5, H6 }

pub enum HeadingDisplay {
    XxLarge,
    Large,
    Medium,
    Small,
    XxSmall,
    Normal,
    Subtitle,
}

pub struct Heading {
    renderable  : fn() -> bool,
    weight      : isize,
    id          : IdentifierValue,
    classes     : Classes,
    heading_type: HeadingType,
    html        : Markup,
    display     : HeadingDisplay,
    template    : String,
}

impl ComponentTrait for Heading {
    fn new() -> Self {
        Heading {
            renderable  : render_always,
            weight      : 0,
            id          : IdentifierValue::new(),
            classes     : Classes::new(),
            heading_type: HeadingType::H1,
            html        : html! {},
            display     : HeadingDisplay::Normal,
            template    : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        HEADING_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, _: &mut InContext) -> Markup {
        let id = self.id().get();
        let classes = self.classes().get();
        html! { @match &self.heading_type() {
            HeadingType::H1 => h1 id=[id] class=[classes] { (*self.html()) },
            HeadingType::H2 => h2 id=[id] class=[classes] { (*self.html()) },
            HeadingType::H3 => h3 id=[id] class=[classes] { (*self.html()) },
            HeadingType::H4 => h4 id=[id] class=[classes] { (*self.html()) },
            HeadingType::H5 => h5 id=[id] class=[classes] { (*self.html()) },
            HeadingType::H6 => h6 id=[id] class=[classes] { (*self.html()) },
        }}
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Heading {
    pub fn h1(html: Markup) -> Self {
        Heading::new().with_heading_type(HeadingType::H1).with_html(html)
    }

    pub fn h2(html: Markup) -> Self {
        Heading::new().with_heading_type(HeadingType::H2).with_html(html)
    }

    pub fn h3(html: Markup) -> Self {
        Heading::new().with_heading_type(HeadingType::H3).with_html(html)
    }

    pub fn h4(html: Markup) -> Self {
        Heading::new().with_heading_type(HeadingType::H4).with_html(html)
    }

    pub fn h5(html: Markup) -> Self {
        Heading::new().with_heading_type(HeadingType::H5).with_html(html)
    }

    pub fn h6(html: Markup) -> Self {
        Heading::new().with_heading_type(HeadingType::H6).with_html(html)
    }

    // Heading BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
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

    pub fn with_heading_type(mut self, heading_type: HeadingType) -> Self {
        self.alter_heading_type(heading_type);
        self
    }

    pub fn with_html(mut self, html: Markup) -> Self {
        self.alter_html(html);
        self
    }

    pub fn with_display(mut self, display: HeadingDisplay) -> Self {
        self.alter_display(display);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Heading ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
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

    pub fn alter_heading_type(&mut self, heading_type: HeadingType) -> &mut Self {
        self.heading_type = heading_type;
        self
    }

    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.html = html;
        self
    }

    pub fn alter_display(&mut self, display: HeadingDisplay) -> &mut Self {
        self.display = display;
        self.classes.alter(match &self.display() {
            HeadingDisplay::XxLarge  => "display-2",
            HeadingDisplay::Large    => "display-3",
            HeadingDisplay::Medium   => "display-4",
            HeadingDisplay::Small    => "display-5",
            HeadingDisplay::XxSmall  => "display-6",
            HeadingDisplay::Normal   => "",
            HeadingDisplay::Subtitle => "",
        }, ClassesOp::SetDefault);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Paragraph GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn heading_type(&self) -> &HeadingType {
        &self.heading_type
    }

    pub fn html(&self) -> &Markup {
        &self.html
    }

    pub fn display(&self) -> &HeadingDisplay {
        &self.display
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
