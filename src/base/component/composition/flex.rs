use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Flex {
    id           : OptionId,
    weight       : Weight,
    renderable   : Renderable,
    classes      : OptionClasses,
    flex_grow    : FlexGrow,
    flex_shrink  : FlexShrink,
    flex_size    : FlexSize,
    flex_offset  : FlexOffset,
    flex_align   : FlexAlign,
    mixed        : MixedComponents,
}

impl ComponentTrait for Flex {
    fn new() -> Self {
        Flex::default()
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

impl Flex {
    pub fn with(component: impl ComponentTrait) -> Self {
        Flex::default().add_component(component)
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
    pub fn alter_grow(&mut self, grow: FlexGrow) -> &mut Self {
        self.flex_grow = grow;
        self
    }

    #[fn_builder]
    pub fn alter_shrink(&mut self, shrink: FlexShrink) -> &mut Self {
        self.flex_shrink = shrink;
        self
    }

    #[fn_builder]
    // Ensures the item occupies the exact specified width, neither growing nor shrinking,
    // regardless of the available space in the container or the size of other items.
    pub fn alter_size(&mut self, size: FlexSize) -> &mut Self {
        self.flex_size = size;
        self
    }

    #[fn_builder]
    pub fn alter_offset(&mut self, offset: FlexOffset) -> &mut Self {
        self.flex_offset = offset;
        self
    }

    #[fn_builder]
    pub fn alter_align(&mut self, align: FlexAlign) -> &mut Self {
        self.flex_align = align;
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

    // Item GETTERS.

    pub fn grow(&self) -> &FlexGrow {
        &self.flex_grow
    }

    pub fn shrink(&self) -> &FlexShrink {
        &self.flex_shrink
    }

    pub fn size(&self) -> &FlexSize {
        &self.flex_size
    }

    pub fn offset(&self) -> &FlexOffset {
        &self.flex_offset
    }

    pub fn align(&self) -> &FlexAlign {
        &self.flex_align
    }

    pub fn components(&self) -> &MixedComponents {
        &self.mixed
    }
}
