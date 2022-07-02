use crate::base::module::demopage;
use crate::core::module::ModuleTrait;
use crate::core::theme::ThemeTrait;

pub trait AppTrait: Send + Sync {
    fn bootstrap(&self) {
    }

    fn enable_modules(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![
            &demopage::Demopage,
        ]
    }

    fn disable_modules(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![]
    }

    fn themes(&self) -> Vec<&'static dyn ThemeTrait> {
        vec![]
    }
}
