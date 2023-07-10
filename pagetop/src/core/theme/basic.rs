use crate::core::component::ContextOp;
use crate::core::module::ModuleTrait;
use crate::core::theme::{ThemeStaticRef, ThemeTrait};
use crate::html::{Favicon, StyleSheet};
use crate::response::page::Page;
use crate::service;
use crate::{serve_static_files, use_handle, use_static, Handle};

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
    fn before_prepare_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::located("/theme/css/normalize.min.css").with_version("8.0.1"),
            ));
    }
}
