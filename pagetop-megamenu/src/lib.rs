use pagetop::prelude::*;

pub mod component;

new_handle!(MODULE_MEGAMENU);

static_files!(megamenu);

pub struct MegaMenu;

impl ModuleTrait for MegaMenu {
    fn handle(&self) -> Handle {
        MODULE_MEGAMENU
    }

    fn dependencies(&self) -> Vec<ModuleRef> {
        vec![&pagetop_jquery::JQuery]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/megamenu", megamenu);
    }
}
