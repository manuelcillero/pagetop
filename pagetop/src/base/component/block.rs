use crate::prelude::*;

new_handle!(COMPONENT_BASE_BLOCK);

actions_for_component!(Block);

#[rustfmt::skip]
#[derive(Default)]
pub struct Block {
    weight    : Weight,
    renderable: Renderable,
    id        : OptionId,
    classes   : OptionClasses,
    title     : OptionTranslate,
    stuff     : ArcComponents,
    template  : String,
}

impl ComponentTrait for Block {
    fn new() -> Self {
        Block::default().with_classes(ClassesOp::SetDefault, "block")
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_BLOCK
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

    fn before_prepare_component(&mut self, cx: &mut Context) {
        run_actions_before_prepare_block(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let id = cx.required_id::<Block>(self.id());
        PrepareMarkup::With(html! {
            div id=(id) class=[self.classes().get()] {
                @if let Some(title) = self.title().using(cx.langid()) {
                    h2 class="block-title" { (title) }
                }
                div class="block-body" {
                    (self.components().prepare(cx))
                }
            }
        })
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        run_actions_after_prepare_block(self, cx);
    }
}

impl Block {
    // Block BUILDER.

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
    pub fn alter_title(&mut self, title: L10n) -> &mut Self {
        self.title.alter_value(title);
        self
    }

    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.stuff.alter(ArcOp::Add(ArcComponent::with(component)));
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: ArcOp) -> &mut Self {
        self.stuff.alter(op);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Block GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn title(&self) -> &OptionTranslate {
        &self.title
    }

    pub fn components(&self) -> &ArcComponents {
        &self.stuff
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
