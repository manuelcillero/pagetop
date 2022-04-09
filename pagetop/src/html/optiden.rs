pub struct OptIden(Option<String>);

impl OptIden {
    pub fn new() -> Self {
        OptIden(None)
    }

    pub fn new_with_value(id: &str) -> Self {
        let mut option = Self::new();
        option.with_value(id);
        option
    }

    pub fn with_value(&mut self, id: &str) -> &Self {
        let id = id.trim();
        self.0 = match id.is_empty() {
            true => None,
            false => Some(id.replace(" ", "_")),
        };
        self
    }

    pub fn option(&self) -> &Option<String> {
        &self.0
    }
}
