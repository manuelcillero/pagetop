use crate::prelude::*;

#[derive(AutoDefault)]
pub enum ItemType {
    #[default]
    Default,
    Region,
    Wrapper,
    Bundle,
}

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Item {
    id         : OptionId,
    renderable : Renderable,
    classes    : OptionClasses,
    item_type  : ItemType,
    flex_grow  : flex::Grow,
    flex_shrink: flex::Shrink,
    flex_size  : flex::Size,
    flex_offset: flex::Offset,
    flex_align : flex::Align,
    mixed      : MixedComponents,
}

impl ComponentTrait for Item {
    fn new() -> Self {
        Item::default()
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
        let (output, region) = match self.item_type() {
            ItemType::Region => (
                self.components().render(cx),
                if let Some(id) = self.id() {
                    cx.prepare_region(id)
                } else {
                    Markup::default()
                },
            ),
            _ => (self.components().render(cx), Markup::default()),
        };
        if output.is_empty() && region.is_empty() {
            return PrepareMarkup::None;
        }
        match self.item_type() {
            ItemType::Default => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] {
                    div class="flex__content" {
                        (output)
                    }
                }
            }),
            ItemType::Region => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] {
                    div class="flex__content flex__region" {
                        (region)
                        (output)
                    }
                }
            }),
            ItemType::Wrapper => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] {
                    (output)
                }
            }),
            ItemType::Bundle => PrepareMarkup::With(html! {
                (output)
            }),
        }
    }
}

impl Item {
    pub fn region() -> Self {
        Item {
            item_type: ItemType::Region,
            ..Default::default()
        }
    }

    pub fn wrapper() -> Self {
        Item {
            item_type: ItemType::Wrapper,
            ..Default::default()
        }
    }

    pub fn bundle() -> Self {
        Item {
            item_type: ItemType::Bundle,
            ..Default::default()
        }
    }

    pub fn with(component: impl ComponentTrait) -> Self {
        Item::default().add_component(component)
    }

    // Item BUILDER.

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
    pub fn alter_grow(&mut self, grow: flex::Grow) -> &mut Self {
        self.flex_grow = grow;
        self
    }

    #[fn_builder]
    pub fn alter_shrink(&mut self, shrink: flex::Shrink) -> &mut Self {
        self.flex_shrink = shrink;
        self
    }

    #[fn_builder]
    // Ensures the item occupies the exact specified width, neither growing nor shrinking,
    // regardless of the available space in the container or the size of other items.
    pub fn alter_size(&mut self, size: flex::Size) -> &mut Self {
        self.flex_size = size;
        self
    }

    #[fn_builder]
    pub fn alter_offset(&mut self, offset: flex::Offset) -> &mut Self {
        self.flex_offset = offset;
        self
    }

    #[fn_builder]
    pub fn alter_align(&mut self, align: flex::Align) -> &mut Self {
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

    pub fn item_type(&self) -> &ItemType {
        &self.item_type
    }

    pub fn grow(&self) -> &flex::Grow {
        &self.flex_grow
    }

    pub fn shrink(&self) -> &flex::Shrink {
        &self.flex_shrink
    }

    pub fn size(&self) -> &flex::Size {
        &self.flex_size
    }

    pub fn offset(&self) -> &flex::Offset {
        &self.flex_offset
    }

    pub fn align(&self) -> &flex::Align {
        &self.flex_align
    }

    pub fn components(&self) -> &MixedComponents {
        &self.mixed
    }
}
