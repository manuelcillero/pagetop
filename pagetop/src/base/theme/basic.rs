use crate::prelude::*;
use crate::BaseHandle;

#[derive(BaseHandle)]
pub struct Basic;

impl ModuleTrait for Basic {
    fn name(&self) -> L10n {
        L10n::n("Basic")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Basic)
    }
}

impl ThemeTrait for Basic {
    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/base/css/normalize.min.css")
                    .with_version("8.0.1")
                    .with_weight(-90),
            ))
            .alter_context(ContextOp::AddBaseAssets)
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/base/css/basic.css")
                    .with_version("0.0.1")
                    .with_weight(-90),
            ));
    }
}
