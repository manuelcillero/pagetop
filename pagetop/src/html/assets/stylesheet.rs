use super::AssetsTrait;
use crate::html::{html, Markup};

pub enum TargetMedia {
    Default,
    Print,
    Screen,
    Speech,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct StyleSheet {
    source : String,
    prefix : &'static str,
    version: &'static str,
    media  : Option<&'static str>,
    weight : isize,
}

impl AssetsTrait for StyleSheet {
    fn source(&self) -> &str {
        self.source.as_str()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn render(&self) -> Markup {
        html! {
            link
                rel="stylesheet"
                href=(crate::concat_string!(self.source, self.prefix, self.version))
                media=[self.media];
        }
    }
}

impl StyleSheet {
    pub fn located<S>(source: S) -> Self
    where
        S: Into<String>,
    {
        StyleSheet {
            source: source.into(),
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

    #[rustfmt::skip]
    pub fn for_media(mut self, media: TargetMedia) -> Self {
        self.media = match media {
            TargetMedia::Print  => Some("print"),
            TargetMedia::Screen => Some("screen"),
            TargetMedia::Speech => Some("speech"),
            _ => None,
        };
        self
    }
}
