use crate::prelude::*;

pub const MINIMAL_THEME: &str = "pagetop::theme::minimal";

pub struct Minimal;

impl ThemeTrait for Minimal {
    fn handler(&self) -> &'static str {
        MINIMAL_THEME
    }
}
