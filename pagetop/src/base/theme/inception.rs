use crate::prelude::*;

new_handle!(THEME_INCEPTION);

static_files!(base);

pub struct Inception;

impl ModuleTrait for Inception {
    fn handle(&self) -> Handle {
        THEME_INCEPTION
    }

    fn name(&self) -> L10n {
        L10n::n("Default")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Inception)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, "/base", base);
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
            .alter_context(ContextOp::AddAssetsForBase)
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/base/css/styles.css")
                    .with_version("0.0.1")
                    .with_weight(-90),
            ));
    }
}
