use pagetop::prelude::*;

define_handle!(MODULE_ADMIN);

define_locale!(LOCALE_ADMIN, "src/locales");

mod summary;

pub struct Admin;

impl ModuleTrait for Admin {
    fn handle(&self) -> Handle {
        MODULE_ADMIN
    }

    fn name(&self) -> String {
        _t("module_name", Locale::From(&LOCALE_ADMIN))
    }

    fn description(&self) -> Option<String> {
        Some(_t("module_description", Locale::From(&LOCALE_ADMIN)))
    }

    #[rustfmt::skip]
    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![
            &pagetop_minimal::Minimal,
            &pagetop_megamenu::MegaMenu,
        ]
    }

    fn actions(&self) -> Vec<HookAction> {
        vec![hook_action!(BeforeRenderPageHook => before_render_page)]
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(
            server::web::scope("/admin").route("", server::web::get().to(summary::summary)),
        );
    }
}

fn before_render_page(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-admin");
}
