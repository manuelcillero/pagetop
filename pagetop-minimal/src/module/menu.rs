use pagetop::prelude::*;

use_handle!(MODULE_MENU);

use_locale!(LOCALE_MENU, "src/module/menu/locales");

pub struct Menu;

impl ModuleTrait for Menu {
    fn handle(&self) -> Handle {
        MODULE_MENU
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALE_MENU)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALE_MENU)
    }
}
