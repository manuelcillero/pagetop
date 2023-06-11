use pagetop::prelude::*;

define_handle!(COMPONENT_PARAGRAPH);

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
    components: ComponentsBundle,
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

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        html! {
            p
                id=[self.id()]
                class=[self.classes().get()]
            {
                (self.components().render(rcx))
            }
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

    #[fn_builder]
    pub fn alter_component(&mut self, component: impl ComponentTrait) -> &mut Self {
        self.components.add(component);
        self
    }

    #[fn_builder]
    pub fn alter_bundle(&mut self, op: BundleOp, component: impl ComponentTrait) -> &mut Self {
        self.components.alter_bundle(op, component);
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

    pub fn components(&self) -> &ComponentsBundle {
        &self.components
    }

    pub fn display(&self) -> &ParagraphDisplay {
        &self.display
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
