use crate::core::module::ModuleTrait;
use crate::core::theme::{ThemeStaticRef, ThemeTrait};
use crate::html::Favicon;
use crate::response::page::Page;
use crate::{define_handle, serve_static_files, service, Handle};

define_handle!(THEME_BASIC);

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

pub struct Basic;

impl ModuleTrait for Basic {
    fn handle(&self) -> Handle {
        THEME_BASIC
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Basic)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/theme", bundle_theme);
    }
}

impl ThemeTrait for Basic {
    fn before_render_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")));
    }
}
