use pagetop::prelude::*;

pub_const_handler!(MODULE_ADMIN);

localize!("src/locales");

mod summary;

pub struct Admin;

impl ModuleTrait for Admin {
    fn handler(&self) -> Handler {
        MODULE_ADMIN
    }

    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.service(app::web::scope("/admin").route("", app::web::get().to(summary::summary)));
    }

    fn actions(&self) -> Vec<HookAction> {
        vec![hook_action!(BeforeRenderPageHook => before_render_page)]
    }
}

fn before_render_page(page: &mut Page) {
    page.alter_body_classes(ClassesOp::Add, "test-admin");
}
