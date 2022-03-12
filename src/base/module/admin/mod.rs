use crate::prelude::*;

localize!("en-US", "src/base/module/admin/locales");

mod summary;

pub struct AdminModule;

impl Module for AdminModule {
    fn name(&self) -> &'static str {
        "admin"
    }

    fn fullname(&self) -> String {
        l("module_fullname")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(
            server::web::scope("/admin")
                .route("", server::web::get().to(summary::summary))
        );
    }
}
