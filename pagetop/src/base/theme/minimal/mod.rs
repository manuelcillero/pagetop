use crate::prelude::*;

pub struct MinimalTheme;

impl ThemeTrait for MinimalTheme {
    fn name(&self) -> &'static str {
        "Minimal"
    }

    fn fullname(&self) -> String {
        "Minimal".to_owned()
    }
}
