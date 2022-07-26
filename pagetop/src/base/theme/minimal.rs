use crate::prelude::*;

pub_const_handler!(THEME_MINIMAL);

pub struct Minimal;

impl ThemeTrait for Minimal {
    fn handler(&self) -> Handler {
        THEME_MINIMAL
    }
}
