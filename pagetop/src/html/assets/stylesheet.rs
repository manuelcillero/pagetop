use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};
use crate::Weight;

pub enum TargetMedia {
    Default,
    Print,
    Screen,
    Speech,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct StyleSheet {
    path   : String,
    prefix : &'static str,
    version: &'static str,
    media  : Option<&'static str>,
    weight : Weight,
}

impl AssetsTrait for StyleSheet {
    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn prepare(&self) -> Markup {
        html! {
            link
                rel="stylesheet"
                href=(crate::concat_string!(self.path, self.prefix, self.version))
                media=[self.media];
        }
    }
}

impl StyleSheet {
    pub fn at<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        StyleSheet {
            path: path.into(),
            ..Default::default()
        }
    }

    pub fn with_version(mut self, version: &'static str) -> Self {
        (self.prefix, self.version) = if version.is_empty() {
            ("", "")
        } else {
            ("?v=", version)
        };
        self
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
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
