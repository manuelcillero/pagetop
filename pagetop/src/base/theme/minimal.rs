use crate::prelude::*;

pub_handle!(THEME_MINIMAL);

pub struct Minimal;

impl ThemeTrait for Minimal {
    fn handle(&self) -> Handle {
        THEME_MINIMAL
    }
}
