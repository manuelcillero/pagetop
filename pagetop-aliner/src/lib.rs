use pagetop::prelude::*;

define_handle!(THEME_ALINER);

include!(concat!(env!("OUT_DIR"), "/aliner.rs"));

pub struct Aliner;

impl ModuleTrait for Aliner {
    fn handle(&self) -> Handle {
        THEME_ALINER
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Aliner)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/aliner", bundle_aliner);
    }
}

impl ThemeTrait for Aliner {
    fn before_render_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::located("/aliner/css/styles.css").with_weight(-99),
            ));
    }
}
