use pagetop::prelude::*;

pub_handle!(MODULE_ADMIN);

pub_locale!("src/locales");

mod summary;

pub struct Admin;

impl ModuleTrait for Admin {
    fn handle(&self) -> Handle {
        MODULE_ADMIN
    }

    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(
            server::web::scope("/admin").route("", server::web::get().to(summary::summary)),
        );
    }

    fn actions(&self) -> Vec<HookAction> {
        vec![hook_action!(BeforeRenderPageHook => before_render_page)]
    }
}

fn before_render_page(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-admin");
}
