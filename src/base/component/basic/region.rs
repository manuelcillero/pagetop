use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Region(OptionId);

impl ComponentTrait for Region {
    fn new() -> Self {
        Region::default()
    }

    fn id(&self) -> Option<String> {
        self.0.get()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.id() {
            Some(id) => PrepareMarkup::With(cx.prepare_region(id)),
            _ => PrepareMarkup::None,
        }
    }
}

impl Region {
    // Region BUILDER.

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.0.alter_value(id);
        self
    }
}
