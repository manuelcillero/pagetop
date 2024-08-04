use crate::prelude::*;

pub struct Inception;

impl PackageTrait for Inception {
    fn name(&self) -> L10n {
        L10n::n("Inception")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Inception)
    }
}

impl ThemeTrait for Inception {
    fn after_prepare_body(&self, page: &mut Page) {
        page.set_assets(AssetsOp::SetFavicon(
            Some(Favicon::new().with_icon("/base/favicon.ico")),
        ))
        .set_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/base/css/normalize.min.css")
                .with_version("8.0.1")
                .with_weight(-90),
        ))
        .set_assets(AssetsOp::AddBaseAssets)
        .set_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/base/css/inception.css")
                .with_version("0.0.1")
                .with_weight(-90),
        ));
    }
}
