use crate::prelude::*;

#[rustfmt::skip]
#[derive(SmartDefault)]
pub struct Item {
    id           : OptionId,
    weight       : Weight,
    renderable   : Renderable,
    item_classes : OptionClasses,
    inner_classes: OptionClasses,
    item_grow    : flex::ItemGrow,
    item_shrink  : flex::ItemShrink,
    item_size    : flex::ItemSize,
    item_offset  : flex::ItemOffset,
    item_align   : flex::ItemAlign,
    stuff        : AnyComponents,
}

impl_handle!(COMPONENT_BASE_FLEX_ITEM for Item);

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
        self.prepend_classes(
            [
                "pt-flex__item".to_owned(),
                self.item_grow.to_string(),
                self.item_shrink.to_string(),
                self.item_size.to_string(),
                self.item_offset.to_string(),
                self.item_align.to_string(),
            ]
            .join(" "),
        );
        self.inner_classes
            .alter_value(ClassesOp::Prepend, "pt-flex__item-inner");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let order = match self.weight() {
            0 => None,
            _ => Some(concat_string!("order: ", self.weight().to_string(), ";")),
        };
        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] style=[order] {
                div class=[self.inner_classes().get()] {
                    (self.components().render(cx))
                }
            }
        })
    }
}

impl ComponentClasses for Item {
    fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.item_classes.alter_value(op, classes);
        self
    }

    fn classes(&self) -> &OptionClasses {
        &self.item_classes
    }
}

impl Item {
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
    pub fn alter_inner_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.inner_classes.alter_value(op, classes);
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

    // Item GETTERS.

    pub fn inner_classes(&self) -> &OptionClasses {
        &self.inner_classes
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

    pub fn components(&self) -> &AnyComponents {
        &self.stuff
    }
}
