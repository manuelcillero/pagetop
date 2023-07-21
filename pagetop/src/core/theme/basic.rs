use crate::core::component::ContextOp;
use crate::core::module::ModuleTrait;
use crate::core::theme::{ThemeRef, ThemeTrait};
use crate::html::{Favicon, StyleSheet};
use crate::response::page::Page;
use crate::service;
use crate::{create_handle, serve_static_files, static_files, Handle};

create_handle!(THEME_BASIC);

static_files!(theme);

pub struct Basic;

impl ModuleTrait for Basic {
    fn handle(&self) -> Handle {
        THEME_BASIC
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Basic)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/theme", theme);
    }
}

impl ThemeTrait for Basic {
    fn before_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/theme/css/normalize.min.css")
                    .with_version("8.0.1")
                    .with_weight(-99),
            ));
    }
}
