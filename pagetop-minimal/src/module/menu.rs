use pagetop::prelude::*;

create_handle!(MODULE_MENU);

static_locales!(LOCALES_MENU in "src/module/menu/locales");

pub struct Menu;

impl ModuleTrait for Menu {
    fn handle(&self) -> Handle {
        MODULE_MENU
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_MENU)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_MENU)
    }
}
