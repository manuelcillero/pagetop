use crate::prelude::*;

pub struct Basic;

impl PackageTrait for Basic {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Basic)
    }
}

impl ThemeTrait for Basic {}
