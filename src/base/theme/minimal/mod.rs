use crate::prelude::*;

pub struct MinimalTheme;

impl Theme for MinimalTheme {
    fn id(&self) -> &'static str {
        "minimal"
    }

    fn name(&self) -> String {
        "Minimal".to_string()
    }
}
