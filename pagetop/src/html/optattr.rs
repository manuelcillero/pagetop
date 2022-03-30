pub struct OptAttr(Option<String>);

impl OptAttr {
    pub fn none() -> Self {
        OptAttr(None)
    }

    pub fn some(value: &str) -> Self {
        let mut o = OptAttr(None);
        o.with_value(value);
        o
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
