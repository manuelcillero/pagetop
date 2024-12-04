use crate::html::Markup;
use crate::locale::{L10n, LanguageIdentifier};
use crate::{fn_builder, AutoDefault};

#[derive(AutoDefault)]
pub struct OptionTranslated(L10n);

impl OptionTranslated {
    pub fn new(value: L10n) -> Self {
        OptionTranslated(value)
    }

    // OptionTranslated BUILDER.

    #[fn_builder]
    pub fn set_value(&mut self, value: L10n) -> &mut Self {
        self.0 = value;
        self
    }

    // OptionTranslated GETTERS.

    pub fn using(&self, langid: &LanguageIdentifier) -> Option<String> {
        self.0.using(langid)
    }

    pub fn escaped(&self, langid: &LanguageIdentifier) -> Markup {
        self.0.escaped(langid)
    }
}
