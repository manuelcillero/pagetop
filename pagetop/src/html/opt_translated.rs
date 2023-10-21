use crate::fn_builder;
use crate::html::{html, Markup};
use crate::locale::{L10n, LanguageIdentifier};

#[derive(Default)]
pub struct OptionTranslated(Option<L10n>);

impl OptionTranslated {
    pub fn new() -> Self {
        OptionTranslated::default()
    }

    pub fn with(value: L10n) -> Self {
        OptionTranslated(Some(value))
    }

    // OptionTranslated BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: L10n) -> &mut Self {
        self.0 = Some(value);
        self
    }

    // OptionTranslated GETTERS.

    pub fn using(&self, langid: &LanguageIdentifier) -> Option<String> {
        if let Some(value) = &self.0 {
            return value.using(langid);
        }
        None
    }

    pub fn escaped(&self, langid: &LanguageIdentifier) -> Markup {
        if let Some(value) = &self.0 {
            return value.escaped(langid);
        }
        html! {}
    }
}
