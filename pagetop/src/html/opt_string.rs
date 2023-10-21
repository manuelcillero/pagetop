use crate::fn_builder;

#[derive(Default)]
pub struct OptionString(Option<String>);

impl OptionString {
    pub fn new() -> Self {
        OptionString::default()
    }

    pub fn with(value: impl Into<String>) -> Self {
        let mut opt = OptionString::default();
        opt.alter_value(value);
        opt
    }

    // OptionString BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: impl Into<String>) -> &mut Self {
        self.0 = Some(value.into().trim().to_owned());
        self
    }

    // OptionString GETTERS.

    pub fn get(&self) -> Option<String> {
        if let Some(value) = &self.0 {
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
        None
    }
}
