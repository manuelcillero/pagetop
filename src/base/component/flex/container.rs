use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Container {
    id             : OptionId,
    weight         : Weight,
    renderable     : Renderable,
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

    fn setup_before_prepare(&mut self, cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                self.direction().to_string(),
                self.wrap_align().to_string(),
                self.content_justify().to_string(),
                self.items_align().to_string(),
            ]
            .join(" "),
        );

        cx.set_param::<bool>(PARAM_BASE_INCLUDE_FLEX_ASSETS, true);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let gap = match self.gap() {
            flex::Gap::Default => None,
            _ => Some(self.gap().to_string()),
        };

        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] style=[gap] {
                (self.items().render(cx))
            }
        })
    }
}

impl Container {
    // Container BUILDER.

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

    #[rustfmt::skip]
    pub fn add_item(mut self, item: flex::Item) -> Self {
        self.items.alter_value(ArcTypedOp::Add(ArcTypedComponent::new(item)));
        self
    }

    #[fn_builder]
    pub fn alter_items(&mut self, op: ArcTypedOp<flex::Item>) -> &mut Self {
        self.items.alter_value(op);
        self
    }

    #[fn_builder]
    pub fn alter_direction(&mut self, direction: flex::Direction) -> &mut Self {
        self.direction = direction;
        self
    }

    #[fn_builder]
    pub fn alter_wrap_align(&mut self, wrap: flex::WrapAlign) -> &mut Self {
        self.wrap_align = wrap;
        self
    }

    #[fn_builder]
    pub fn alter_content_justify(&mut self, justify: flex::ContentJustify) -> &mut Self {
        self.content_justify = justify;
        self
    }

    #[fn_builder]
    pub fn alter_items_align(&mut self, align: flex::ItemAlign) -> &mut Self {
        self.items_align = align;
        self
    }

    #[fn_builder]
    pub fn alter_gap(&mut self, gap: flex::Gap) -> &mut Self {
        self.gap = gap;
        self
    }

    // Container GETTERS.

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
