use crate::prelude::*;

#[rustfmt::skip]
#[derive(Default)]
pub struct Paragraph {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    classes   : OptionClasses,
    font_size : FontSize,
    stuff     : AnyComponents,
}

impl_handle!(COMPONENT_BASE_PARAGRAPH for Paragraph);

impl ComponentTrait for Paragraph {
    fn new() -> Self {
        Paragraph::default()
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
        self.classes.alter_value(ClassesOp::AddFirst, self.font_size.to_string());
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            p
                id=[self.id()]
                class=[self.classes().get()]
            {
                (self.components().render(cx))
            }
        })
    }
}

impl Paragraph {
    pub fn with(component: impl ComponentTrait) -> Self {
        Paragraph::default().add_component(component)
    }

    pub fn translated(l10n: L10n) -> Self {
        Paragraph::default().add_translated(l10n)
    }

    // Paragraph BUILDER.

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
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_font_size(&mut self, font_size: FontSize) -> &mut Self {
        self.font_size = font_size;
        self
    }

    #[rustfmt::skip]
    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.stuff.alter_value(ArcAnyOp::Add(ArcAnyComponent::new(component)));
        self
    }

    #[rustfmt::skip]
    pub fn add_translated(mut self, l10n: L10n) -> Self {
        self.stuff.alter_value(ArcAnyOp::Add(ArcAnyComponent::new(Translate::with(l10n))));
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: ArcAnyOp) -> &mut Self {
        self.stuff.alter_value(op);
        self
    }

    // Paragraph GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn font_size(&self) -> &FontSize {
        &self.font_size
    }

    pub fn components(&self) -> &AnyComponents {
        &self.stuff
    }
}
