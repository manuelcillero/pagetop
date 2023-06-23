use crate::prelude::*;

use_handle!(THEME_BASIC);

use_static!(theme);

pub struct Basic;

impl ModuleTrait for Basic {
    fn handle(&self) -> Handle {
        THEME_BASIC
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Basic)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/theme", theme);
    }
}

impl ThemeTrait for Basic {
    fn before_render_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")));
    }
}
