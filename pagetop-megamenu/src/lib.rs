use pagetop::prelude::*;

pub mod component {
    mod item;
    pub use item::{MegaMenuItem, MegaMenuItemType, COMPONENT_MEGAMENUITEM};
    mod menu;
    pub use menu::{MegaMenu, COMPONENT_MEGAMENU};
}

new_handle!(MODULE_MEGAMENU);

static_locales!(LOCALES_MEGAMENU);

static_files!(megamenu);

pub struct MegaMenu;

impl ModuleTrait for MegaMenu {
    fn handle(&self) -> Handle {
        MODULE_MEGAMENU
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_MEGAMENU)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_MEGAMENU)
    }

    fn dependencies(&self) -> Vec<ModuleRef> {
        vec![&pagetop_jquery::JQuery]
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        serve_static_files!(scfg, "/megamenu", megamenu);
    }
}
