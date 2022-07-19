use super::{AssetsTrait, SourceValue};
use crate::html::{html, Markup};

pub struct StyleSheet {
    source : SourceValue,
    prefix : &'static str,
    version: &'static str,
    weight : isize,
}

impl AssetsTrait for StyleSheet {
    fn source(&self) -> SourceValue {
        self.source
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn render(&self) -> Markup {
        html! {
            link
                rel="stylesheet"
                href=(crate::concat_string!(self.source, self.prefix, self.version));
        }
    }
}

impl StyleSheet {
    pub fn located(source: SourceValue) -> Self {
        StyleSheet {
            source,
            prefix : "",
            version: "",
            weight : 0,
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
}
