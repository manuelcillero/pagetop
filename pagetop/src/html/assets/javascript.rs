use super::AssetsTrait;
use crate::html::{html, Markup};

#[derive(Default, Eq, PartialEq)]
pub enum ModeJS {
    Async,
    #[default]
    Defer,
    Normal,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct JavaScript {
    source : String,
    weight : isize,
    mode   : ModeJS,
}

impl AssetsTrait for JavaScript {
    fn source(&self) -> &str {
        self.source.as_str()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn prepare(&self) -> Markup {
        html! {
            script type="text/javascript"
                src=(self.source)
                async[self.mode == ModeJS::Async]
                defer[self.mode == ModeJS::Defer]
                {};
        }
    }
}

impl JavaScript {
    pub fn located<S>(source: S) -> Self
    where
        S: Into<String>,
    {
        JavaScript {
            source: source.into(),
            ..Default::default()
        }
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
