pub struct OptIden(Option<String>);

impl OptIden {
    pub fn none() -> Self {
        OptIden(None)
    }

    pub fn some(id: &str) -> Self {
        let mut o = OptIden(None);
        o.with_value(id);
        o
    }

    pub fn with_value(&mut self, id: &str) -> &mut Self {
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
