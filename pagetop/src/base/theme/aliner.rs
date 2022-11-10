use crate::prelude::*;

pub_handle!(THEME_ALINER);

include!(concat!(env!("OUT_DIR"), "/aliner.rs"));

pub struct Aliner;

impl ThemeTrait for Aliner {
    fn handle(&self) -> Handle {
        THEME_ALINER
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        configure_service_for_static_files!(cfg, "/aliner", bundle_aliner);
    }

    fn before_render_page(&self, page: &mut Page) {
        page.alter_context(PageOp::AddFavicon(
            Favicon::new().with_icon("/theme/favicon.png"),
        ))
        .alter_context(PageOp::AddStyleSheet(
            StyleSheet::located("/aliner/css/styles.css").with_weight(-99),
        ));
    }
}
