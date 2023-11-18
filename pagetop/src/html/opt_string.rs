use crate::{fn_builder, SmartDefault};

#[derive(SmartDefault)]
pub struct OptionString(Option<String>);

impl OptionString {
    pub fn new(value: impl Into<String>) -> Self {
        OptionString::default().with_value(value)
    }

    // OptionString BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.0 = Some(value.into().trim().to_owned());
        self
    }

    // OptionString GETTERS.

    pub fn get(&self) -> Option<String> {
        if let Some(value) = &self.0 {
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
        None
    }
}
