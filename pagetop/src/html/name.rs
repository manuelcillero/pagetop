use crate::fn_builder;

#[derive(Default)]
pub struct NameValue(String);

impl NameValue {
    pub fn new() -> Self {
        NameValue::default()
    }

    // NameValue BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.0 = value.trim().replace(' ', "_");
        self
    }

    // NameValue GETTERS.

    pub fn get(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.to_owned())
        }
    }
}
