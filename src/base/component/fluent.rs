use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Fluent(L10n);

impl ComponentTrait for Fluent {
    fn new() -> Self {
        Fluent::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(self.l10n().escaped(cx.langid()))
    }
}

impl Fluent {
    pub fn with(l10n: L10n) -> Self {
        Fluent(l10n)
    }

    // Fluent BUILDER.

    #[fn_builder]
    pub fn alter_l10n(&mut self, l10n: L10n) -> &mut Self {
        self.0 = l10n;
        self
    }

    // Fluent GETTERS.

    pub fn l10n(&self) -> &L10n {
        &self.0
    }
}
