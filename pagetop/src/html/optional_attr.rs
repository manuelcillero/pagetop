pub struct OptionAttr(Option<String>);

impl OptionAttr {
    pub fn none() -> Self {
        OptionAttr(None)
    }

    pub fn some(value: &str) -> Self {
        let value = value.trim();
        match value.is_empty() {
            true => OptionAttr(None),
            false => OptionAttr(Some(value.to_owned())),
        }
    }

    pub fn with_value(&mut self, value: &str) {
        let value = value.trim();
        self.0 = match value.is_empty() {
            true => None,
            false => Some(value.to_owned()),
        };
    }

    pub fn value(&self) -> &str {
        match &self.0 {
            Some(value) => value.as_str(),
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
