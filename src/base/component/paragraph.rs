use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Paragraph {
    id       : OptionId,
    classes  : OptionClasses,
    font_size: FontSize,
    mixed    : MixedComponents,
}

impl ComponentTrait for Paragraph {
    fn new() -> Self {
        Paragraph::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, self.font_size().to_string());
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

    pub fn fluent(l10n: L10n) -> Self {
        Paragraph::default().add_component(Fluent::with(l10n))
    }

    // Paragraph BUILDER.

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_font_size(&mut self, font_size: FontSize) -> &mut Self {
        self.font_size = font_size;
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: AnyOp) -> &mut Self {
        self.mixed.alter_value(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.mixed.alter_value(AnyOp::Add(AnyComponent::with(component)));
        self
    }

    // Paragraph GETTERS.

    pub fn font_size(&self) -> &FontSize {
        &self.font_size
    }

    pub fn components(&self) -> &MixedComponents {
        &self.mixed
    }
}
