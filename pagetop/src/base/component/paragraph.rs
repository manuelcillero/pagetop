use crate::prelude::*;

pub const COMPONENT_PARAGRAPH: &str = "pagetop::component::paragraph";

pub enum ParagraphDisplay {
    XxLarge,
    Large,
    Medium,
    Small,
    XxSmall,
    Normal,
}

pub struct Paragraph {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    html      : Markup,
    display   : ParagraphDisplay,
    template  : String,
}

impl ComponentTrait for Paragraph {
    fn new() -> Self {
        Paragraph {
            weight    : 0,
            renderable: render_always,
            id        : IdentifierValue::new(),
            classes   : Classes::new(),
            html      : html! {},
            display   : ParagraphDisplay::Normal,
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        COMPONENT_PARAGRAPH
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &InContext) -> bool {
        (self.renderable)(context)
    }

    fn default_render(&self, _: &mut InContext) -> Markup {
        html! {
            p id=[self.id().get()] class=[self.classes().get()] { (*self.html()) }
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

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
        self
    }

    pub fn with_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_classes(op, classes);
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

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Paragraph ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, renderable: Renderable) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter(op, classes);
        self
    }

    pub fn alter_html(&mut self, html: Markup) -> &mut Self {
        self.html = html;
        self
    }

    pub fn alter_display(&mut self, display: ParagraphDisplay) -> &mut Self {
        self.display = display;
        self.classes.alter(ClassesOp::SetDefault, match &self.display() {
            ParagraphDisplay::XxLarge => "fs-2",
            ParagraphDisplay::Large   => "fs-3",
            ParagraphDisplay::Medium  => "fs-4",
            ParagraphDisplay::Small   => "fs-5",
            ParagraphDisplay::XxSmall => "fs-6",
            ParagraphDisplay::Normal  => "",
        });
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

    pub fn html(&self) -> &Markup {
        &self.html
    }

    pub fn display(&self) -> &ParagraphDisplay {
        &self.display
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
