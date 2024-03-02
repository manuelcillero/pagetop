use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Block {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    classes   : OptionClasses,
    title     : OptionTranslated,
    stuff     : AnyComponents,
}

impl ComponentTrait for Block {
    fn new() -> Self {
        Block::default()
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

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.prepend_classes("block__container");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let block_body = self.components().render(cx);
        if !block_body.is_empty() {
            let id = cx.required_id::<Block>(self.id());
            return PrepareMarkup::With(html! {
                div id=(id) class=[self.classes().get()] {
                    @if let Some(title) = self.title().using(cx.langid()) {
                        h2 class="block__title" { (title) }
                    }
                    div class="block__body" { (block_body) }
                }
            });
        }
        PrepareMarkup::None
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
    pub fn alter_title(&mut self, title: L10n) -> &mut Self {
        self.title.alter_value(title);
        self
    }

    #[rustfmt::skip]
    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.stuff.alter_value(ArcAnyOp::Add(ArcAnyComponent::new(component)));
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: ArcAnyOp) -> &mut Self {
        self.stuff.alter_value(op);
        self
    }

    // Block GETTERS.

    pub fn title(&self) -> &OptionTranslated {
        &self.title
    }

    pub fn components(&self) -> &AnyComponents {
        &self.stuff
    }
}
