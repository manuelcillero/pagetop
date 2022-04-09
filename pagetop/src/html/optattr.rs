pub struct OptAttr(Option<String>);

impl OptAttr {
    pub fn new() -> Self {
        OptAttr(None)
    }

    pub fn new_with_value(value: &str) -> Self {
        let mut option = Self::new();
        option.with_value(value);
        option
    }

    pub fn with_value(&mut self, value: &str) -> &mut Self {
        let value = value.trim();
        self.0 = match value.is_empty() {
            true => None,
            false => Some(value.to_owned()),
        };
        self
    }

    pub fn option(&self) -> &Option<String> {
        &self.0
    }
}
