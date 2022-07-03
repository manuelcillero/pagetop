pub struct AttributeValue(String);

impl AttributeValue {
    pub fn new() -> Self {
        AttributeValue("".to_owned())
    }

    pub fn new_with_value(value: &str) -> Self {
        let mut attr = Self::new();
        attr.with_value(value);
        attr
    }

    pub fn with_value(&mut self, value: &str) -> &mut Self {
        self.0 = value.trim().to_owned();
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
