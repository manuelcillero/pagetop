use pagetop::prelude::*;

define_handle!(MODULE_MENU);

define_locale!(LOCALE_MENU, "src/module/menu/locales");

pub struct Menu;

impl ModuleTrait for Menu {
    fn handle(&self) -> Handle {
        MODULE_MENU
    }

    fn name(&self) -> String {
        t("module_name", Locale::From(&LOCALE_MENU))
    }

    fn description(&self) -> Option<String> {
        Some(t("module_description", Locale::From(&LOCALE_MENU)))
    }
}
