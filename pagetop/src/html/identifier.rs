#[derive(Default)]
pub struct IdentifierValue(String);

impl IdentifierValue {
    pub fn new() -> Self {
        IdentifierValue("".to_owned())
    }

    pub fn new_with_value(value: &str) -> Self {
        let mut id = Self::new();
        id.with_value(value);
        id
    }

    pub fn with_value(&mut self, value: &str) -> &Self {
        self.0 = value.trim().replace(' ', "_");
        self
    }

    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.to_owned())
        }
    }
}
