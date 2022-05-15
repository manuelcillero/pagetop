use crate::html::{Markup, html};

pub struct StyleSheet {
    pub(super) source: &'static str,
    pub(super) weight: isize,
}
impl StyleSheet {
    pub fn source(s: &'static str) -> Self {
        StyleSheet {
            source: s,
            weight: 0,
        }
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }

    pub fn weight(self) -> isize {
        self.weight
    }

    pub(super) fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" href=(self.source);
        }
    }
}
