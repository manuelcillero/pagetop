use crate::prelude::*;

pub_const_handler!(THEME_ALINER);

include!(concat!(env!("OUT_DIR"), "/aliner.rs"));

pub struct Aliner;

impl ThemeTrait for Aliner {
    fn handler(&self) -> Handler {
        THEME_ALINER
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        theme_static_files!(cfg, "/aliner");
    }

    fn before_render_page(&self, page: &mut Page) {
        page.alter_context(InContextOp::Favicon(Some(
            Favicon::new().with_icon("/theme/favicon.png"),
        )))
        .alter_context(InContextOp::StyleSheet(AssetsOp::Add(
            StyleSheet::located("/aliner/css/styles.css").with_weight(-99),
        )));
    }
}
