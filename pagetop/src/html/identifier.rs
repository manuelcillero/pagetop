#[derive(Default)]
pub struct IdentifierValue(String);

impl IdentifierValue {
    pub fn new() -> Self {
        IdentifierValue::default()
    }

    // IdentifierValue BUILDER.

    pub fn with_value(mut self, value: &str) -> Self {
        self.alter_value(value);
        self
    }

    // IdentifierValue ALTER.

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
