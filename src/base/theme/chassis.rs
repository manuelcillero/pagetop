use crate::prelude::*;

pub struct Chassis;

impl PackageTrait for Chassis {
    fn name(&self) -> L10n {
        L10n::n("Chassis")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Chassis)
    }
}

impl ThemeTrait for Chassis {
    fn after_prepare_body(&self, page: &mut Page) {
        page.set_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")))
            .set_assets(AssetsOp::AddStyleSheet(
                StyleSheet::from("/base/css/normalize.min.css")
                    .with_version("8.0.1")
                    .with_weight(-90),
            ))
            .set_assets(AssetsOp::AddBaseAssets)
            .set_assets(AssetsOp::AddStyleSheet(
                StyleSheet::from("/base/css/chassis.css")
                    .with_version("0.0.1")
                    .with_weight(-90),
            ));
    }
}
