use crate::prelude::*;

pub const THEME_MINIMAL: &str = "pagetop::theme::minimal";

pub struct Minimal;

impl ThemeTrait for Minimal {
    fn handler(&self) -> &'static str {
        THEME_MINIMAL
    }
}
