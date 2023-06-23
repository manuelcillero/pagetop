use pagetop::prelude::*;

use_handle!(MODULE_ADMIN);

define_locale!(LOCALE_ADMIN, "src/locale");

mod summary;

pub struct Admin;

impl ModuleTrait for Admin {
    fn handle(&self) -> Handle {
        MODULE_ADMIN
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALE_ADMIN)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALE_ADMIN)
    }

    #[rustfmt::skip]
    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![
            &pagetop_minimal::Minimal,
            &pagetop_megamenu::MegaMenu,
        ]
    }

    fn actions(&self) -> Vec<Action> {
        vec![action!(ActionBeforeRenderPage => before_render_page)]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.service(
            service::web::scope("/admin").route("", service::web::get().to(summary::summary)),
        );
    }
}

fn before_render_page(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-admin");
}
