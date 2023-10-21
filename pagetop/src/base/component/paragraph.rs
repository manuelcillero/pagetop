use crate::prelude::*;

new_handle!(COMPONENT_BASE_PARAGRAPH);

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
    weight    : Weight,
    renderable: Renderable,
    id        : OptionId,
    classes   : OptionClasses,
    stuff     : ArcComponents,
    display   : ParagraphDisplay,
    template  : String,
}

impl ComponentTrait for Paragraph {
    fn new() -> Self {
        Paragraph::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_PARAGRAPH
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
        Paragraph::new().add_component(component)
    }

    pub fn translated(l10n: L10n) -> Self {
        Paragraph::new().add_translated(l10n)
    }

    // Paragraph BUILDER.

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

    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.stuff.alter(ArcOp::Add(ArcComponent::with(component)));
        self
    }

    pub fn add_translated(mut self, l10n: L10n) -> Self {
        self.stuff
            .alter(ArcOp::Add(ArcComponent::with(Translate::with(l10n))));
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: ArcOp) -> &mut Self {
        self.stuff.alter(op);
        self
    }

    #[rustfmt::skip]
    #[fn_builder]
    pub fn alter_display(&mut self, display: ParagraphDisplay) -> &mut Self {
        self.alter_classes(
            ClassesOp::SetDefault,
            match display {
                ParagraphDisplay::XxLarge => "fs-2",
                ParagraphDisplay::Large   => "fs-3",
                ParagraphDisplay::Medium  => "fs-4",
                ParagraphDisplay::Small   => "fs-5",
                ParagraphDisplay::XxSmall => "fs-6",
                ParagraphDisplay::Normal  => "",
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

    pub fn components(&self) -> &ArcComponents {
        &self.stuff
    }

    pub fn display(&self) -> &ParagraphDisplay {
        &self.display
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
