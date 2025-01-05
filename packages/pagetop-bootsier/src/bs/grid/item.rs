use pagetop::prelude::*;

use crate::bs::grid;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Item {
    id        : OptionId,
    classes   : OptionClasses,
    columns   : grid::ItemColumns,
    responsive: grid::ItemResponsive,
    start     : grid::ItemStart,
    children  : Children,
}

impl ComponentTrait for Item {
    fn new() -> Self {
        Item::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                self.columns().to_string(),
                self.responsive().to_string(),
                self.start().to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = self.children().render(cx);
        if output.is_empty() {
            return PrepareMarkup::None;
        }
        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] {
                (output)
            }
        })
    }
}

impl Item {
    pub fn with(child: impl ComponentTrait) -> Self {
        Item::default().with_child(child)
    }

    // Item BUILDER.

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
    pub fn with_columns(mut self, columns: grid::ItemColumns) -> Self {
        self.columns = columns;
        self
    }

    #[fn_builder]
    pub fn with_responsive(mut self, responsive: grid::ItemResponsive) -> Self {
        self.responsive = responsive;
        self
    }

    #[fn_builder]
    pub fn with_start(mut self, start: grid::ItemStart) -> Self {
        self.start = start;
        self
    }

    pub fn with_child(mut self, child: impl ComponentTrait) -> Self {
        self.children.add(Child::with(child));
        self
    }

    #[fn_builder]
    pub fn with_children(mut self, op: ChildOp) -> Self {
        self.children.alter_child(op);
        self
    }

    // Item GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn columns(&self) -> &grid::ItemColumns {
        &self.columns
    }

    pub fn responsive(&self) -> &grid::ItemResponsive {
        &self.responsive
    }

    pub fn start(&self) -> &grid::ItemStart {
        &self.start
    }

    pub fn children(&self) -> &Children {
        &self.children
    }
}
