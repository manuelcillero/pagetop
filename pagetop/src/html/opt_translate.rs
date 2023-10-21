use crate::fn_builder;
use crate::html::{html, Markup};
use crate::locale::{L10n, LanguageIdentifier};

#[derive(Default)]
pub struct OptionTranslate(Option<L10n>);

impl OptionTranslate {
    pub fn new() -> Self {
        OptionTranslate::default()
    }

    pub fn with(value: L10n) -> Self {
        OptionTranslate(Some(value))
    }

    // OptionTranslate BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, value: L10n) -> &mut Self {
        self.0 = Some(value);
        self
    }

    // OptionTranslate GETTERS.

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
