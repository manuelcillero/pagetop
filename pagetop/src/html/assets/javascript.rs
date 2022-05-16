use crate::html::{Markup, html};
use super::AssetsTrait;

#[derive(PartialEq)]
pub enum JSMode { Async, Defer, Normal }

pub struct JavaScript {
    source: &'static str,
    weight: isize,
    mode  : JSMode,
}

impl AssetsTrait for JavaScript {
    fn source(&self) -> &'static str {
        self.source
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn render(&self) -> Markup {
        html! {
            script type="text/javascript"
                src=(self.source)
                async[self.mode == JSMode::Async]
                defer[self.mode == JSMode::Defer]
                {};
        }
    }
}

impl JavaScript {
    pub fn with_source(s: &'static str) -> Self {
        JavaScript {
            source: s,
            weight: 0,
            mode  : JSMode::Defer,
        }
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_mode(mut self, mode: JSMode) -> Self {
        self.mode = mode;
        self
    }
}
