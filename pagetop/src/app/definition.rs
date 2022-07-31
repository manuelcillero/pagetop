use crate::core::module::ModuleStaticRef;
use crate::core::theme::ThemeStaticRef;

pub trait AppTrait: Send + Sync {
    fn bootstrap(&self) {}

    fn enable_modules(&self) -> Vec<ModuleStaticRef> {
        vec![]
    }

    fn disable_modules(&self) -> Vec<ModuleStaticRef> {
        vec![]
    }

    fn themes(&self) -> Vec<ThemeStaticRef> {
        vec![]
    }
}
