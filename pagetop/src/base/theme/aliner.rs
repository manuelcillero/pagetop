use crate::prelude::*;

pub const ALINER_THEME: &str = "pagetop::theme::aliner";

include!(concat!(env!("OUT_DIR"), "/aliner.rs"));

pub struct Aliner;

impl ThemeTrait for Aliner {
    fn handler(&self) -> &'static str {
        ALINER_THEME
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        theme_static_files!(cfg, "/aliner");
    }

    fn before_render_page(&self, page: &mut Page) {
        page.context()
            .with_favicon(Some(Favicon::new()
                .with_icon("/theme/favicon.png")
            ))
            .add_stylesheet(
                StyleSheet::with_source(
                    "/aliner/css/styles.css"
                )
                .with_weight(-99)
            );
    }
}
