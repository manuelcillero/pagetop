use crate::core::module::ModuleTrait;
use crate::core::theme::ThemeTrait;

pub trait AppTrait: Send + Sync {
    fn bootstrap(&self) {
    }

    fn enabled_modules(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![]
    }

    fn disabled_modules(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![]
    }

    fn register_themes(&self) -> Vec<&'static dyn ThemeTrait> {
        vec![]
    }
}
