use pagetop::prelude::*;

pub mod component;
pub mod module;

define_handle!(MODULE_MINIMAL);

include!(concat!(env!("OUT_DIR"), "/minimal.rs"));

pub struct Minimal;

impl ModuleTrait for Minimal {
    fn handle(&self) -> Handle {
        MODULE_MINIMAL
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/minimal", bundle_minimal);
    }
}
