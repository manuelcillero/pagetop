use super::AssetsTrait;
use crate::html::{html, Markup};

#[derive(Default, PartialEq)]
pub enum ModeJS {
    Async,
    #[default]
    Defer,
    Normal,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct JavaScript {
    source : &'static str,
    prefix : &'static str,
    version: &'static str,
    weight : isize,
    mode   : ModeJS,
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
                src=(crate::concat_string!(self.source, self.prefix, self.version))
                async[self.mode == ModeJS::Async]
                defer[self.mode == ModeJS::Defer]
                {};
        }
    }
}

impl JavaScript {
    pub fn located(source: &'static str) -> Self {
        JavaScript {
            source,
            ..Default::default()
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

    pub fn with_mode(mut self, mode: ModeJS) -> Self {
        self.mode = mode;
        self
    }
}
