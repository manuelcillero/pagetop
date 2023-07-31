use pagetop::prelude::*;

new_handle!(THEME_ALINER);

static_files!(aliner);

pub struct Aliner;

impl ModuleTrait for Aliner {
    fn handle(&self) -> Handle {
        THEME_ALINER
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Aliner)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        serve_static_files!(scfg, "/aliner", aliner);
    }
}

impl ThemeTrait for Aliner {
    fn before_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/aliner/css/styles.css").with_weight(-99),
            ));
    }
}
