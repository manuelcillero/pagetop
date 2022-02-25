use crate::prelude::*;
use crate::base::module::admin::summary::summary;

pub struct AdminModule;

impl Module for AdminModule {
    fn name(&self) -> String {
        "PageTop Admin".to_string()
    }

    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(
            server::web::scope("/admin")
                .route("", server::web::get().to(summary))
        );
    }
}
