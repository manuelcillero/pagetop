use crate::prelude::*;

pub_handle!(THEME_SATURN);

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

pub struct Saturn;

impl ModuleTrait for Saturn {
    fn handle(&self) -> Handle {
        THEME_SATURN
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Saturn)
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        serve_static_files!(cfg, "/theme", bundle_theme);
    }
}

impl ThemeTrait for Saturn {
    fn before_render_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::located("/theme/css/styles.css").with_weight(-99),
            ));
    }
}
