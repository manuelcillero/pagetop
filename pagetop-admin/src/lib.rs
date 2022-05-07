use pagetop::prelude::*;

pub const ADMIN_MODULE: &str = "pagetop-admin::module::admin";

localize!("src/locales");

mod summary;

pub struct Admin;

impl ModuleTrait for Admin {
    fn handler(&self) -> &'static str {
        ADMIN_MODULE
    }

    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.service(
            app::web::scope("/admin")
                .route("", app::web::get().to(summary::summary))
        );
    }

    fn actions(&self) -> Vec<ActionItem> {
        vec![
            action_item!(ActionBeforeRenderPage => before_render_page)
        ]
    }
}

fn before_render_page(page: &mut Page) {
    page.alter_body_classes("test-admin", ClassesOp::Add);
}
