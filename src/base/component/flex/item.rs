use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Item {
    id           : OptionId,
    weight       : Weight,
    renderable   : Renderable,
    classes      : OptionClasses,
    item_wide    : flex::ItemWide,
    item_grow    : flex::ItemGrow,
    item_shrink  : flex::ItemShrink,
    item_size    : flex::ItemSize,
    item_offset  : flex::ItemOffset,
    item_align   : flex::ItemAlign,
    mixed        : MixedComponents,
}

impl ComponentTrait for Item {
    fn new() -> Self {
        Item::default()
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
        self.alter_classes(
            ClassesOp::Prepend,
            [
                String::from("flex__item"),
                self.wide().to_string(),
                self.grow().to_string(),
                self.shrink().to_string(),
                self.size().to_string(),
                self.offset().to_string(),
                self.align().to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = self.components().render(cx);
        if !output.is_empty() {
            let order = match self.weight() {
                0 => None,
                _ => Some(concat_string!("order: ", self.weight().to_string(), ";")),
            };
            PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] style=[order] {
                    div class="flex__content" {
                        (output)
                    }
                }
            })
        } else {
            PrepareMarkup::None
        }
    }
}

impl Item {
    pub fn with(component: impl ComponentTrait) -> Self {
        Item::default().add_component(component)
    }

    pub fn full(component: impl ComponentTrait) -> Self {
        Item::default()
            .with_wide(flex::ItemWide::Full)
            .add_component(component)
    }

    // Item BUILDER.

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
    pub fn alter_wide(&mut self, wide: flex::ItemWide) -> &mut Self {
        self.item_wide = wide;
        self
    }

    #[fn_builder]
    pub fn alter_grow(&mut self, grow: flex::ItemGrow) -> &mut Self {
        self.item_grow = grow;
        self
    }

    #[fn_builder]
    pub fn alter_shrink(&mut self, shrink: flex::ItemShrink) -> &mut Self {
        self.item_shrink = shrink;
        self
    }

    #[fn_builder]
    // Ensures the item occupies the exact specified width, neither growing nor shrinking,
    // regardless of the available space in the container or the size of other items.
    pub fn alter_size(&mut self, size: flex::ItemSize) -> &mut Self {
        self.item_size = size;
        self
    }

    #[fn_builder]
    pub fn alter_offset(&mut self, offset: flex::ItemOffset) -> &mut Self {
        self.item_offset = offset;
        self
    }

    #[fn_builder]
    pub fn alter_align(&mut self, align: flex::ItemAlign) -> &mut Self {
        self.item_align = align;
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: MixedOp) -> &mut Self {
        self.mixed.alter_value(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.mixed.alter_value(MixedOp::Add(AnyComponent::with(component)));
        self
    }

    // Item GETTERS.

    pub fn wide(&self) -> &flex::ItemWide {
        &self.item_wide
    }

    pub fn grow(&self) -> &flex::ItemGrow {
        &self.item_grow
    }

    pub fn shrink(&self) -> &flex::ItemShrink {
        &self.item_shrink
    }

    pub fn size(&self) -> &flex::ItemSize {
        &self.item_size
    }

    pub fn offset(&self) -> &flex::ItemOffset {
        &self.item_offset
    }

    pub fn align(&self) -> &flex::ItemAlign {
        &self.item_align
    }

    pub fn components(&self) -> &MixedComponents {
        &self.mixed
    }
}
