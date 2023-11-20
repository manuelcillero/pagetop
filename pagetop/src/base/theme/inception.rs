use crate::prelude::*;
use crate::BaseHandle;

#[derive(BaseHandle)]
pub struct Inception;

impl ModuleTrait for Inception {
    fn name(&self) -> L10n {
        L10n::n("Inception")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Inception)
    }
}

impl ThemeTrait for Inception {
    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/base/css/normalize.min.css")
                    .with_version("8.0.1")
                    .with_weight(-90),
            ))
            .alter_context(ContextOp::AddBaseAssets)
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/base/css/inception.css")
                    .with_version("0.0.1")
                    .with_weight(-90),
            ));
    }
}
