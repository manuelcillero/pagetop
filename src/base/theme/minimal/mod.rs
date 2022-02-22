use crate::prelude::*;

pub struct MinimalTheme;

impl Theme for MinimalTheme {
    fn name(&self) -> String {
        "Minimal".to_string()
    }
}
