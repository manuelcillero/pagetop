//! Es el tema básico que incluye PageTop por defecto.

use crate::prelude::*;

/// Tema básico por defecto.
pub struct Basic;

impl Extension for Basic {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }
}

impl Theme for Basic {
    fn after_render_page_body(&self, page: &mut Page) {
        page.alter_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/css/normalize.css")
                .with_version("8.0.1")
                .with_weight(-99),
        ))
        .alter_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/css/basic.css")
                .with_version(env!("CARGO_PKG_VERSION"))
                .with_weight(-99),
        ));
    }
}
