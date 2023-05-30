use pagetop::prelude::*;

pub mod component;

define_handle!(MODULE_MEGAMENU);

include!(concat!(env!("OUT_DIR"), "/megamenu.rs"));

pub struct MegaMenu;

impl ModuleTrait for MegaMenu {
    fn handle(&self) -> Handle {
        MODULE_MEGAMENU
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![&pagetop_jquery::JQuery, &pagetop_minimal::Minimal]
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        serve_static_files!(cfg, "/megamenu", bundle_megamenu);
    }
}
