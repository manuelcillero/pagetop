use crate::html::{Markup, html};
use super::AssetsTrait;

pub struct StyleSheet {
    source: &'static str,
    weight: isize,
}

impl AssetsTrait for StyleSheet {
    fn source(&self) -> &'static str {
        self.source
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" href=(self.source);
        }
    }
}

impl StyleSheet {
    pub fn with_source(s: &'static str) -> Self {
        StyleSheet {
            source: s,
            weight: 0,
        }
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }
}
