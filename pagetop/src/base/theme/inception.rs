use crate::prelude::*;

new_handle!(THEME_INCEPTION);

static_files!(theme);

const VERSION_INCEPTION: &str = "0.0.0";

pub struct InceptionTheme;

impl ModuleTrait for InceptionTheme {
    fn handle(&self) -> Handle {
        THEME_INCEPTION
    }

    fn name(&self) -> L10n {
        L10n::n("Default")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&InceptionTheme)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, "/theme", theme);
    }
}

impl ThemeTrait for InceptionTheme {
    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/theme/css/normalize.min.css")
                    .with_version("8.0.1")
                    .with_weight(-99),
            ))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/theme/css/root.css").with_version(VERSION_INCEPTION),
            ));

        if let Some(true) = page.context().get_param::<bool>(PARAM_INCLUDE_FLEX) {
            page.alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/theme/css/flex.css").with_version(VERSION_INCEPTION),
            ));
        }
        if let Some(true) = page.context().get_param::<bool>(PARAM_INCLUDE_ICONS) {
            page.alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/theme/icons/bootstrap-icons.css").with_version("1.8.2"),
            ));
        }
        page.alter_context(ContextOp::AddStyleSheet(
            StyleSheet::at("/theme/css/styles.css").with_version(VERSION_INCEPTION),
        ));
    }
}
