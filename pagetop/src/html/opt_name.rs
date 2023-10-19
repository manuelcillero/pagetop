use crate::fn_builder;

#[derive(Default)]
pub struct OptionName(String);

impl OptionName {
    pub fn new() -> Self {
        OptionName::default()
    }

    pub fn with(value: impl Into<String>) -> Self {
        let mut opt = OptionName::default();
        opt.alter_value(value);
        opt
    }

    // OptionName BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.0 = value.into().trim().replace(' ', "_");
        self
    }

    // OptionName GETTERS.

    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.to_owned())
        }
    }
}
