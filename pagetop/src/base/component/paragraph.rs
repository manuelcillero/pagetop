use crate::prelude::*;

pub const PARAGRAPH_COMPONENT: &str = "pagetop::component::paragraph";

pub enum ParagraphDisplay {
    XxLarge,
    Large,
    Medium,
    Small,
    XxSmall,
    Normal,
}

pub struct Paragraph {
    renderable: fn() -> bool,
    weight    : isize,
    html      : Markup,
    display   : ParagraphDisplay,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl ComponentTrait for Paragraph {
    fn new() -> Self {
        Paragraph {
            renderable: render_always,
            weight    : 0,
            html      : html! {},
            display   : ParagraphDisplay::Normal,
            id        : OptIden::new(),
            classes   : Classes::new(),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        PARAGRAPH_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, _: &mut Assets) -> Markup {
        html! {
            p id=[self.id()] class=[self.classes()] { (*self.html()) }
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Paragraph {
    pub fn with(html: Markup) -> Self {
        Paragraph::new().with_html(html)
    }

    // Paragraph BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_html(mut self, html: Markup) -> Self {
        self.alter_html(html);
        self
    }

    pub fn with_display(mut self, display: ParagraphDisplay) -> Self {
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

    // Paragraph ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.html = html;
        self
    }

    pub fn alter_display(&mut self, display: ParagraphDisplay) -> &mut Self {
        self.display = display;
        self.classes.alter(match &self.display() {
            ParagraphDisplay::XxLarge => "fs-2",
            ParagraphDisplay::Large   => "fs-3",
            ParagraphDisplay::Medium  => "fs-4",
            ParagraphDisplay::Small   => "fs-5",
            ParagraphDisplay::XxSmall => "fs-6",
            ParagraphDisplay::Normal  => "",
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

    pub fn html(&self) -> &Markup {
        &self.html
    }

    pub fn display(&self) -> &ParagraphDisplay {
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
