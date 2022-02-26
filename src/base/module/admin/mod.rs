use crate::prelude::*;

localize!("en-US", "src/base/module/admin/locales");

mod summary;

pub struct AdminModule;

impl Module for AdminModule {
    fn id(&self) -> &'static str {
        "admin"
    }

    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> String {
        l("module_desc")
    }

    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(
            server::web::scope("/admin")
                .route("", server::web::get().to(summary::summary))
        );
    }
}
