use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Block {
    id        : OptionId,
    renderable: Renderable,
    classes   : OptionClasses,
    style     : StyleBase,
    title     : OptionTranslated,
    mixed     : MixedComponents,
}

impl ComponentTrait for Block {
    fn new() -> Self {
        Block::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [String::from("block__container"), self.style().to_string()].join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let block_body = self.components().render(cx);

        if block_body.is_empty() {
            return PrepareMarkup::None;
        }

        let id = cx.required_id::<Block>(self.id());

        PrepareMarkup::With(html! {
            div id=(id) class=[self.classes().get()] {
                @if let Some(title) = self.title().using(cx.langid()) {
                    h2 class="block__title" { (title) }
                }
                div class="block__content" { (block_body) }
            }
        })
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
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_style(&mut self, style: StyleBase) -> &mut Self {
        self.style = style;
        self
    }

    #[fn_builder]
    pub fn alter_title(&mut self, title: L10n) -> &mut Self {
        self.title.alter_value(title);
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

    // Block GETTERS.

    pub fn style(&self) -> &StyleBase {
        &self.style
    }

    pub fn title(&self) -> &OptionTranslated {
        &self.title
    }

    pub fn components(&self) -> &MixedComponents {
        &self.mixed
    }
}
