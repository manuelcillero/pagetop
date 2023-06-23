use pagetop::prelude::*;

use_handle!(THEME_ALINER);

use_static!(aliner);

pub struct Aliner;

impl ModuleTrait for Aliner {
    fn handle(&self) -> Handle {
        THEME_ALINER
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Aliner)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/aliner", aliner);
    }
}

impl ThemeTrait for Aliner {
    fn before_prepare_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::located("/aliner/css/styles.css").with_weight(-99),
            ));
    }
}
