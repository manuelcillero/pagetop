use pagetop::prelude::*;

define_handle!(MODULE_MENU);

define_locale!("src/module/menu/locales");

pub struct Menu;

impl ModuleTrait for Menu {
    fn handle(&self) -> Handle {
        MODULE_MENU
    }

    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }
}
