use pagetop::prelude::*;

use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Dropdown {
    id     : AttrId,
    classes: AttrClasses,
    items  : Children,
}

impl Component for Dropdown {
    fn new() -> Self {
        Dropdown::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, "dropdown");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let items = self.items().render(cx);
        if items.is_empty() {
            return PrepareMarkup::None;
        }

        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] {
                button
                    type="button"
                    class="btn btn-secondary dropdown-toggle"
                    data-bs-toggle="dropdown"
                    aria-expanded="false"
                {
                    ("Dropdown button")
                }
                ul class="dropdown-menu" {
                    li {
                        a class="dropdown-item" href="#" {
                            ("Action")
                        }
                    }
                    li {
                        a class="dropdown-item" href="#" {
                            ("Another action")
                        }
                    }
                    li {
                        a class="dropdown-item" href="#" {
                            ("Something else here")
                        }
                    }
                }
            }
        })
    }
}

impl Dropdown {
    // **< Dropdown BUILDER >***********************************************************************

    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    pub fn add_item(mut self, item: dropdown::Item) -> Self {
        self.items.add(Child::with(item));
        self
    }

    #[builder_fn]
    pub fn with_items(mut self, op: TypedOp<dropdown::Item>) -> Self {
        self.items.alter_typed(op);
        self
    }

    // **< Dropdown GETTERS >***********************************************************************

    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    pub fn items(&self) -> &Children {
        &self.items
    }
}
