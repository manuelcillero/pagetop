use pagetop::prelude::*;

pub_handle!(THEME_MINIMAL);

pub struct Minimal;

impl ModuleTrait for Minimal {
    fn handle(&self) -> Handle {
        THEME_MINIMAL
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Minimal)
    }
}

impl ThemeTrait for Minimal {}
