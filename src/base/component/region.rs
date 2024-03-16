use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Region(OptionId);

impl ComponentTrait for Region {
    fn new() -> Self {
        Region::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        if let Some(name) = self.name().get() {
            return PrepareMarkup::With(cx.prepare_region(name.as_str()));
        }
        PrepareMarkup::None
    }
}

impl Region {
    pub fn named(name: impl Into<String>) -> Self {
        Region::new().with_name(name)
    }

    // Region BUILDER.

    #[fn_builder]
    pub fn alter_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.0.alter_value(name);
        self
    }

    // Region GETTERS.

    pub fn name(&self) -> &OptionId {
        &self.0
    }
}
