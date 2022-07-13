use crate::html::{Markup, html};
use super::{AssetsTrait, SourceValue};

#[derive(PartialEq)]
pub enum JSMode { Async, Defer, Normal }

pub struct JavaScript {
    source : SourceValue,
    prefix : &'static str,
    version: &'static str,
    weight : isize,
    mode   : JSMode,
}

impl AssetsTrait for JavaScript {
    fn source(&self) -> SourceValue {
        self.source
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn render(&self) -> Markup {
        html! {
            script type="text/javascript"
                src=(crate::concat_string!(self.source, self.prefix, self.version))
                async[self.mode == JSMode::Async]
                defer[self.mode == JSMode::Defer]
                {};
        }
    }
}

impl JavaScript {
    pub fn located(source: SourceValue) -> Self {
        JavaScript {
            source,
            prefix : "",
            version: "",
            weight : 0,
            mode   : JSMode::Defer,
        }
    }

    pub fn with_version(mut self, version: &'static str) -> Self {
        (self.prefix, self.version) = if version.is_empty() {
            ("", "")
        } else {
            ("?ver=", version)
        };
        self
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
