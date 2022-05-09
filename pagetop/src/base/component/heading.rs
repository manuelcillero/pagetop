use crate::prelude::*;

pub const HEADING_COMPONENT: &str = "pagetop::component::heading";

pub enum HeadingType {
    H1(String),
    H2(String),
    H3(String),
    H4(String),
    H5(String),
    H6(String),
}

pub enum HeadingDisplay {
    XxLarge,
    Large,
    Normal,
    Medium,
    Small,
    XxSmall,
}

pub struct Heading {
    renderable: fn() -> bool,
    weight    : isize,
    heading   : HeadingType,
    display   : HeadingDisplay,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl ComponentTrait for Heading {
    fn new() -> Self {
        Heading {
            renderable: render_always,
            weight    : 0,
            heading   : HeadingType::H1("".to_owned()),
            display   : HeadingDisplay::Normal,
            id        : OptIden::new(),
            classes   : Classes::new(),
            template  : "default".to_owned(),
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

    fn default_render(&self, _: &mut Assets) -> Markup {
        html! { @match &self.heading() {
            HeadingType::H1(text) => h1 id=[self.id()] class=[self.classes()] { (text) },
            HeadingType::H2(text) => h2 id=[self.id()] class=[self.classes()] { (text) },
            HeadingType::H3(text) => h3 id=[self.id()] class=[self.classes()] { (text) },
            HeadingType::H4(text) => h4 id=[self.id()] class=[self.classes()] { (text) },
            HeadingType::H5(text) => h5 id=[self.id()] class=[self.classes()] { (text) },
            HeadingType::H6(text) => h6 id=[self.id()] class=[self.classes()] { (text) },
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
    pub fn with(heading: HeadingType) -> Self {
        Heading::new().with_heading(heading)
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

    pub fn with_heading(mut self, heading: HeadingType) -> Self {
        self.alter_heading(heading);
        self
    }

    pub fn with_display(mut self, display: HeadingDisplay) -> Self {
        self.alter_display(display);
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

    pub fn alter_heading(&mut self, heading: HeadingType) -> &mut Self {
        self.heading = heading;
        self
    }

    pub fn alter_display(&mut self, display: HeadingDisplay) -> &mut Self {
        self.display = display;
        self.classes.alter(match &self.display() {
            HeadingDisplay::XxLarge    => "display-2",
            HeadingDisplay::Large      => "display-3",
            HeadingDisplay::Normal     => "",
            HeadingDisplay::Medium     => "display-4",
            HeadingDisplay::Small      => "display-5",
            HeadingDisplay::XxSmall    => "display-6",
        }, ClassesOp::SetDefault);
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

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Paragraph GETTERS.

    pub fn heading(&self) -> &HeadingType {
        &self.heading
    }

    pub fn display(&self) -> &HeadingDisplay {
        &self.display
    }

    pub fn id(&self) -> &Option<String> {
        self.id.option()
    }

    pub fn classes(&self) -> &Option<String> {
        self.classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
