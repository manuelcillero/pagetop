use crate::fn_builder;

#[derive(Default)]
pub struct OptionString(String);

impl OptionString {
    pub fn new() -> Self {
        OptionString::default()
    }

    // OptionString BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.0 = value.into().trim().to_owned();
        self
    }

    // OptionString GETTERS.

    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.to_owned())
        }
    }
}
