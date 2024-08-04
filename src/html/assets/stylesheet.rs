use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};
use crate::{concat_string, AutoDefault, Weight};

#[derive(AutoDefault)]
enum Source {
    #[default]
    From(String),
    Inline(String, String),
}

pub enum TargetMedia {
    Default,
    Print,
    Screen,
    Speech,
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct StyleSheet {
    source : Source,
    prefix : &'static str,
    version: &'static str,
    media  : Option<&'static str>,
    weight : Weight,
}

impl AssetsTrait for StyleSheet {
    fn name(&self) -> &String {
        match &self.source {
            Source::From(path) => path,
            Source::Inline(name, _) => name,
        }
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn prepare(&self) -> Markup {
        match &self.source {
            Source::From(path) => html! {
                link
                    rel="stylesheet"
                    href=(concat_string!(path, self.prefix, self.version))
                    media=[self.media];
            },
            Source::Inline(_, code) => html! {
                styles { (code) };
            },
        }
    }
}

impl StyleSheet {
    pub fn from(path: impl Into<String>) -> Self {
        StyleSheet {
            source: Source::From(path.into()),
            ..Default::default()
        }
    }

    pub fn inline(name: impl Into<String>, styles: impl Into<String>) -> Self {
        StyleSheet {
            source: Source::Inline(name.into(), styles.into()),
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
    pub fn for_media(mut self, media: &TargetMedia) -> Self {
        self.media = match media {
            TargetMedia::Default => None,
            TargetMedia::Print   => Some("print"),
            TargetMedia::Screen  => Some("screen"),
            TargetMedia::Speech  => Some("speech"),
        };
        self
    }
}
