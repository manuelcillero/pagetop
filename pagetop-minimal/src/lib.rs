use pagetop::prelude::*;

pub mod component;
pub mod module;

pub_handle!(MODULE_MINIMAL);

include!(concat!(env!("OUT_DIR"), "/minimal.rs"));

pub struct Minimal;

impl ModuleTrait for Minimal {
    fn handle(&self) -> Handle {
        MODULE_MINIMAL
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        serve_static_files!(cfg, "/minimal", bundle_minimal);
    }
}
