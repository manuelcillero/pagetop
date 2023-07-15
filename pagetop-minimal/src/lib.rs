use pagetop::prelude::*;

pub mod component;
pub mod module;

create_handle!(MODULE_MINIMAL);

static_files!(minimal);

pub struct Minimal;

impl ModuleTrait for Minimal {
    fn handle(&self) -> Handle {
        MODULE_MINIMAL
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/minimal", minimal);
    }
}
