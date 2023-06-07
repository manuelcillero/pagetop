use crate::fn_builder;

#[derive(Default)]
pub struct IdentifierValue(String);

impl IdentifierValue {
    pub fn new() -> Self {
        IdentifierValue::default()
    }

    // IdentifierValue BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.0 = value.trim().replace(' ', "_");
        self
    }

    // IdentifierValue GETTERS.

    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.to_owned())
        }
    }
}
