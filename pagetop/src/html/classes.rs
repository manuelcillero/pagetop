use crate::concat_string;

pub struct Classes(String);

impl Classes {
    pub fn none() -> Self {
        Classes("".to_owned())
    }

    pub fn set_classes(&mut self, classes: &str) -> &mut Self {
        self.0 = classes.to_owned();
        self
    }

    pub fn add_classes(&mut self, classes: &str) -> &mut Self {
        self.0 = concat_string!(self.0, " ", classes).trim().to_owned();
        self
    }

    pub fn option(&self, default: &str) -> Option<String> {
        Some(concat_string!(default.to_owned(), " ", self.0).trim().to_owned())
    }
}
