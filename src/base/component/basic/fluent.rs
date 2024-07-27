use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Fluent(L10n);

impl ComponentTrait for Fluent {
    fn new() -> Self {
        Fluent::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(self.0.escaped(cx.langid()))
    }
}

impl Fluent {
    pub fn with(l10n: L10n) -> Self {
        Fluent(l10n)
    }

    pub fn set_l10n(&mut self, l10n: L10n) -> &mut Self {
        self.0 = l10n;
        self
    }
}
