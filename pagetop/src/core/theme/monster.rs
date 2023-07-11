use crate::core::component::ContextOp;
use crate::core::module::ModuleTrait;
use crate::core::theme::{ThemeStaticRef, ThemeTrait};
use crate::html::{Favicon, StyleSheet};
use crate::response::page::Page;
use crate::service;
use crate::{serve_static_files, use_handle, use_static, Handle};

use_handle!(THEME_MONSTER);

use_static!(monster);

pub struct Monster;

impl ModuleTrait for Monster {
    fn handle(&self) -> Handle {
        THEME_MONSTER
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Monster)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/monster", monster);
    }
}

impl ThemeTrait for Monster {
    fn before_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/monster/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/monster/css/normalize.min.css").with_version("8.0.1"),
            ));
    }
}
