#[derive(Default)]
pub struct AttributeValue(String);

impl AttributeValue {
    pub fn new() -> Self {
        AttributeValue::default()
    }

    // AttributeValue BUILDER.

    pub fn with_value(mut self, value: &str) -> Self {
        self.alter_value(value);
        self
    }

    // AttributeValue ALTER.

    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.0 = value.trim().to_owned();
        self
    }

    // AttributeValue GETTERS.

    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.to_owned())
        }
    }
}
