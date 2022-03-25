use pagetop::prelude::*;

localize!("src/locales");

mod summary;

pub struct AdminModule;

impl ModuleTrait for AdminModule {
    fn name(&self) -> &'static str {
        "Admin"
    }

    fn fullname(&self) -> String {
        l("module_fullname")
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
