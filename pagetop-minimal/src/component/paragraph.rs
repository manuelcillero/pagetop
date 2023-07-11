use pagetop::prelude::*;

use_handle!(COMPONENT_PARAGRAPH);

#[derive(Default)]
pub enum ParagraphDisplay {
    #[default]
    Normal,
    XxLarge,
    Large,
    Medium,
    Small,
    XxSmall,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Paragraph {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    components: PackComponents,
    display   : ParagraphDisplay,
    template  : String,
}

impl ComponentTrait for Paragraph {
    fn new() -> Self {
        Paragraph::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_PARAGRAPH
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            p
                id=[self.id()]
                class=[self.classes().get()]
            {
                (self.components().prepare(cx))
            }
        })
    }
}

impl Paragraph {
    pub fn with(component: impl ComponentTrait) -> Self {
        Paragraph::new().with_component(component)
    }

    // Paragraph BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
        self.components.alter_pack(PackOp::Add, component);
        self
    }

    pub fn alter_components(&mut self, op: PackOp, component: impl ComponentTrait) -> &mut Self {
        self.components.alter_pack(op, component);
        self
    }

    #[rustfmt::skip]
    #[fn_builder]
    pub fn alter_display(&mut self, display: ParagraphDisplay) -> &mut Self {
        self.display = display;
        self.classes.alter_value(
            ClassesOp::SetDefault,
            match &self.display() {
                ParagraphDisplay::XxLarge => "fs-2",
                ParagraphDisplay::Large   => "fs-3",
                ParagraphDisplay::Medium  => "fs-4",
                ParagraphDisplay::Small   => "fs-5",
                ParagraphDisplay::XxSmall => "fs-6",
                ParagraphDisplay::Normal  => "",
            },
        );
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Paragraph GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn components(&self) -> &PackComponents {
        &self.components
    }

    pub fn display(&self) -> &ParagraphDisplay {
        &self.display
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
