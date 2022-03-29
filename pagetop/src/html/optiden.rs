pub struct OptIden(Option<String>);

impl OptIden {
    pub fn none() -> Self {
        OptIden(None)
    }

    pub fn some(id: &str) -> Self {
        let mut o = OptIden::none();
        o.with_value(id);
        o
    }

    pub fn with_value(&mut self, id: &str) {
        let id = id.trim();
        self.0 = match id.is_empty() {
            true => None,
            false => Some(id.replace(" ", "_")),
        };
    }

    pub fn option(&self) -> &Option<String> {
        &self.0
    }
}
