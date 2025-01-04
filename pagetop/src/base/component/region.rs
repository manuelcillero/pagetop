use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Region {
    id     : OptionId,
    classes: OptionClasses,
}

impl ComponentTrait for Region {
    fn new() -> Self {
        Region::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, "region-container");
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = if let Some(id) = self.id() {
            cx.render_region(id)
        } else {
            html! {}
        };
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

impl Region {
    pub fn of(id: impl Into<String>) -> Self {
        Region::default().with_id(id)
    }

    // Region BUILDER.

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

    // Region GETTERS.

    fn classes(&self) -> &OptionClasses {
        &self.classes
    }
}
