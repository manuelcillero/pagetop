use crate::prelude::*;

new_handle!(THEME_CHASSIS);

pub struct Chassis;

impl ModuleTrait for Chassis {
    fn handle(&self) -> Handle {
        THEME_CHASSIS
    }

    fn name(&self) -> L10n {
        L10n::n("Chassis")
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Chassis)
    }
}

impl ThemeTrait for Chassis {
    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/base/css/normalize.min.css")
                    .with_version("8.0.1")
                    .with_weight(-90),
            ))
            .alter_context(ContextOp::AddBaseAssets)
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/base/css/chassis.css")
                    .with_version("0.0.1")
                    .with_weight(-90),
            ));
    }
}
