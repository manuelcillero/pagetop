use crate::html::{html, Markup};

pub struct HtmlMarkup {
    pub markup: Markup,
}

impl Default for HtmlMarkup {
    fn default() -> Self {
        HtmlMarkup { markup: html! {} }
    }
}
