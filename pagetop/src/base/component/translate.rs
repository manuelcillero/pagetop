use crate::prelude::*;
use crate::BaseHandle;

#[derive(BaseHandle, SmartDefault)]
pub struct Translate(L10n);

impl ComponentTrait for Translate {
    fn new() -> Self {
        Translate::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(self.l10n().escaped(cx.langid()))
    }
}

impl Translate {
    pub fn with(l10n: L10n) -> Self {
        Translate(l10n)
    }

    // Translate BUILDER.

    #[fn_with]
    pub fn alter_l10n(&mut self, l10n: L10n) -> &mut Self {
        self.0 = l10n;
        self
    }

    // Translate GETTERS.

    pub fn l10n(&self) -> &L10n {
        &self.0
    }
}
