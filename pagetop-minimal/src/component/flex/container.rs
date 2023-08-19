use pagetop::prelude::*;

use crate::component::flex;

new_handle!(COMPONENT_FLEX_CONTAINER);

actions_for_component!(Container);

#[rustfmt::skip]
#[derive(Default)]
pub struct Container {
    weight    : Weight,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    items     : TypedComponents<flex::Item>,
    template  : String,
}

impl ComponentTrait for Container {
    fn new() -> Self {
        Container::default().with_classes(ClassesOp::SetDefault, "flex")
    }

    fn handle(&self) -> Handle {
        COMPONENT_FLEX_CONTAINER
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
        run_actions_before_prepare_container(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] {
                (self.items().prepare(cx))
            }
        })
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        run_actions_after_prepare_container(self, cx);
    }
}

impl Container {
    // Container BUILDER.

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
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    pub fn with_item(mut self, item: flex::Item) -> Self {
        self.items.alter(TypedOp::Add(TypedComponent::with(item)));
        self
    }

    #[fn_builder]
    pub fn alter_items(&mut self, op: TypedOp<flex::Item>) -> &mut Self {
        self.items.alter(op);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Container GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn items(&self) -> &TypedComponents<flex::Item> {
        &self.items
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
