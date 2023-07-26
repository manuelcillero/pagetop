use crate::core::component::{l10n::L10n, ContextOp};
use crate::core::module::ModuleTrait;
use crate::core::theme::{ThemeRef, ThemeTrait};
use crate::html::{Favicon, StyleSheet};
use crate::response::page::Page;
use crate::service;
use crate::{new_handle, serve_static_files, static_files, Handle};

new_handle!(THEME_DEFAULT);

static_files!(theme);

pub struct DefaultTheme;

impl ModuleTrait for DefaultTheme {
    fn handle(&self) -> Handle {
        THEME_DEFAULT
    }

    fn name(&self) -> L10n {
        L10n::n("Default")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&DefaultTheme)
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/theme", theme);
    }
}

impl ThemeTrait for DefaultTheme {
    fn before_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/theme/css/normalize.min.css")
                    .with_version("8.0.1")
                    .with_weight(-99),
            ));
    }
}
