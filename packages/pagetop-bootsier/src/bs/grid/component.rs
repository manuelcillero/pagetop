use pagetop::prelude::*;

use crate::bs::grid;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Grid {
    id         : OptionId,
    classes    : OptionClasses,
    grid_layout: grid::Layout,
    grid_gap   : grid::Gap,
    items      : Children,
}

impl ComponentTrait for Grid {
    fn new() -> Self {
        Grid::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, "grid");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = self.items().render(cx);
        if output.is_empty() {
            return PrepareMarkup::None;
        }

        let style = option_string!([self.layout().to_string(), self.gap().to_string()]; " ");

        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] style=[style] {
                (output)
            }
        })
    }
}

impl Grid {
    pub fn with(item: grid::Item) -> Self {
        Grid::default().with_item(item)
    }

    // Grid BUILDER.

    #[fn_builder]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl Into<String>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn with_layout(mut self, layout: grid::Layout) -> Self {
        self.grid_layout = layout;
        self
    }

    #[fn_builder]
    pub fn with_gap(mut self, gap: grid::Gap) -> Self {
        self.grid_gap = gap;
        self
    }

    pub fn with_item(mut self, item: grid::Item) -> Self {
        self.items.add(ChildComponent::with(item));
        self
    }

    #[fn_builder]
    pub fn with_items(mut self, op: TypedOp<grid::Item>) -> Self {
        self.items.alter_typed(op);
        self
    }

    // Grid GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn layout(&self) -> &grid::Layout {
        &self.grid_layout
    }

    pub fn gap(&self) -> &grid::Gap {
        &self.grid_gap
    }

    pub fn items(&self) -> &Children {
        &self.items
    }
}
