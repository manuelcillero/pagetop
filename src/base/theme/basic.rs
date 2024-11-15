use crate::prelude::*;

pub struct Basic;

impl PackageTrait for Basic {
    fn name(&self) -> L10n {
        L10n::n("Basic")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Basic)
    }
}

impl ThemeTrait for Basic {
    fn after_prepare_body(&self, page: &mut Page) {
        page.set_assets(AssetsOp::SetFavicon(Some(
            Favicon::new().with_icon("/base/favicon.ico"),
        )))
        .set_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/base/css/normalize.min.css")
                .with_version("8.0.1")
                .with_weight(-90),
        ))
        .set_assets(AssetsOp::AddBaseAssets)
        .set_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/base/css/basic.css")
                .with_version("0.0.1")
                .with_weight(-90),
        ));
    }
}
