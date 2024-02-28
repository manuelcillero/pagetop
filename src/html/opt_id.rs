use crate::{fn_builder, AutoDefault};

#[derive(AutoDefault)]
pub struct OptionId(Option<String>);

impl OptionId {
    pub fn new(value: impl Into<String>) -> Self {
        OptionId::default().with_value(value)
    }

    // OptionId BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.0 = Some(value.into().trim().replace(' ', "_"));
        self
    }

    // OptionId GETTERS.

    pub fn get(&self) -> Option<String> {
        if let Some(value) = &self.0 {
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
        None
    }
}
