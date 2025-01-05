use pagetop::prelude::*;

use crate::bs::navbar;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Nav {
    id     : OptionId,
    classes: OptionClasses,
    items  : Children,
}

impl ComponentTrait for Nav {
    fn new() -> Self {
        Nav::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, "navbar-nav");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let items = self.items().render(cx);
        if items.is_empty() {
            return PrepareMarkup::None;
        }

        PrepareMarkup::With(html! {
            ul id=[self.id()] class=[self.classes().get()] {
                (items)
            }
        })
    }
}

impl Nav {
    // Nav BUILDER.

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

    pub fn with_item(mut self, item: navbar::Item) -> Self {
        self.items.add(Child::with(item));
        self
    }

    #[fn_builder]
    pub fn with_items(mut self, op: TypedOp<navbar::Item>) -> Self {
        self.items.alter_typed(op);
        self
    }

    // Nav GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn items(&self) -> &Children {
        &self.items
    }
}
