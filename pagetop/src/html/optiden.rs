pub struct OptIden(Option<String>);

impl OptIden {
    pub fn none() -> Self {
        OptIden(None)
    }

    pub fn with_value(&mut self, id: &str) {
        let id = id.trim();
        self.0 = match id.is_empty() {
            true => None,
            false => Some(id.replace(" ", "_")),
        };
    }

    pub fn value(&self) -> &str {
        match &self.0 {
            Some(id) => id.as_str(),
            None => "",
        }
    }

    pub fn has_value(&self) -> bool {
        match &self.0 {
            Some(_) => true,
            None => false,
        }
    }

    pub fn option(&self) -> &Option<String> {
        &self.0
    }
}
