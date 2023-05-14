use crate::core::module::{ModuleTrait, ThemeStaticRef, ThemeTrait};
use crate::pub_handle;
use crate::util::Handle;

pub_handle!(THEME_BASIC);

pub struct Basic;

impl ModuleTrait for Basic {
    fn handle(&self) -> Handle {
        THEME_BASIC
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Basic)
    }
}

impl ThemeTrait for Basic {}
