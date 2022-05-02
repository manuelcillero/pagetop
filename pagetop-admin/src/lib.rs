use pagetop::prelude::*;

localize!("src/locales");

mod summary;

pub struct Admin;

impl ModuleTrait for Admin {
    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_module(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.service(
            app::web::scope("/admin")
                .route("", app::web::get().to(summary::summary))
        );
    }
}
