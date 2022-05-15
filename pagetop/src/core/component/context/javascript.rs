use crate::html::{Markup, html};

#[derive(PartialEq)]
pub enum JSMode { Async, Defer, Normal }

pub struct JavaScript {
    pub(super) source: &'static str,
    pub(super) weight: isize,
    pub(super) mode  : JSMode,
}
impl JavaScript {
    pub fn source(s: &'static str) -> Self {
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

    pub fn weight(self) -> isize {
        self.weight
    }

    pub(super) fn render(&self) -> Markup {
        html! {
            script type="text/javascript"
                src=(self.source)
                async[self.mode == JSMode::Async]
                defer[self.mode == JSMode::Defer]
                {};
        }
    }
}
