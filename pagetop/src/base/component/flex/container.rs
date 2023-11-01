use crate::prelude::*;

new_handle!(COMPONENT_BASE_FLEX_CONTAINER);

actions_for_component!(Container);

#[rustfmt::skip]
#[derive(Default)]
pub struct Container {
    weight         : Weight,
    renderable     : Renderable,
    id             : OptionId,
    classes        : OptionClasses,
    items          : TypedComponents<flex::Item>,
    direction      : flex::Direction,
    wrap_align     : flex::WrapAlign,
    content_justify: flex::ContentJustify,
    items_align    : flex::ItemAlign,
    gap            : flex::Gap,
}

impl ComponentTrait for Container {
    fn new() -> Self {
        Container::default()
            .with_classes(ClassesOp::AddDefault, flex::Direction::Default.to_string())
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_FLEX_CONTAINER
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
        cx.set_param::<bool>(PARAM_BASE_INCLUDE_FLEX_ASSETS, true);

        let gap = match self.gap() {
            flex::Gap::Default => None,
            _ => Some(self.gap().to_string()),
        };

        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] style=[gap] {
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
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    pub fn add_item(mut self, item: flex::Item) -> Self {
        self.items.alter(TypedOp::Add(TypedComponent::with(item)));
        self
    }

    #[fn_builder]
    pub fn alter_items(&mut self, op: TypedOp<flex::Item>) -> &mut Self {
        self.items.alter(op);
        self
    }

    #[fn_builder]
    pub fn alter_direction(&mut self, direction: flex::Direction) -> &mut Self {
        self.classes.alter_value(
            ClassesOp::Replace(self.direction.to_string()),
            direction.to_string(),
        );
        self.direction = direction;
        self
    }

    #[fn_builder]
    pub fn alter_wrap_align(&mut self, wrap: flex::WrapAlign) -> &mut Self {
        self.classes.alter_value(
            ClassesOp::Replace(self.wrap_align.to_string()),
            wrap.to_string(),
        );
        self.wrap_align = wrap;
        self
    }

    #[fn_builder]
    pub fn alter_content_justify(&mut self, justify: flex::ContentJustify) -> &mut Self {
        self.classes.alter_value(
            ClassesOp::Replace(self.content_justify.to_string()),
            justify.to_string(),
        );
        self.content_justify = justify;
        self
    }

    #[fn_builder]
    pub fn alter_items_align(&mut self, align: flex::ItemAlign) -> &mut Self {
        self.classes.alter_value(
            ClassesOp::Replace(self.items_align.to_string()),
            align.to_string(),
        );
        self.items_align = align;
        self
    }

    #[fn_builder]
    pub fn alter_gap(&mut self, gap: flex::Gap) -> &mut Self {
        self.gap = gap;
        self
    }

    // Container GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn items(&self) -> &TypedComponents<flex::Item> {
        &self.items
    }

    pub fn direction(&self) -> &flex::Direction {
        &self.direction
    }

    pub fn wrap_align(&self) -> &flex::WrapAlign {
        &self.wrap_align
    }

    pub fn content_justify(&self) -> &flex::ContentJustify {
        &self.content_justify
    }

    pub fn items_align(&self) -> &flex::ItemAlign {
        &self.items_align
    }

    pub fn gap(&self) -> &flex::Gap {
        &self.gap
    }
}
