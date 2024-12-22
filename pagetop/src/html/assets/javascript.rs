use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};
use crate::{join_string, AutoDefault, Weight};

#[derive(AutoDefault)]
enum Source {
    #[default]
    From(String),
    Defer(String),
    Async(String),
    Inline(String, String),
    OnLoad(String, String),
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct JavaScript {
    source : Source,
    prefix : &'static str,
    version: &'static str,
    weight : Weight,
}

impl AssetsTrait for JavaScript {
    fn name(&self) -> &String {
        match &self.source {
            Source::From(path) => path,
            Source::Defer(path) => path,
            Source::Async(path) => path,
            Source::Inline(name, _) => name,
            Source::OnLoad(name, _) => name,
        }
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn render(&self) -> Markup {
        match &self.source {
            Source::From(path) => html! {
                script src=(join_string!(path, self.prefix, self.version)) {};
            },
            Source::Defer(path) => html! {
                script src=(join_string!(path, self.prefix, self.version)) defer {};
            },
            Source::Async(path) => html! {
                script src=(join_string!(path, self.prefix, self.version)) async {};
            },
            Source::Inline(_, code) => html! {
                script { (code) };
            },
            Source::OnLoad(_, code) => html! { (join_string!(
                "document.addEventListener('DOMContentLoaded',function(){",
                code,
                "});"
            )) },
        }
    }
}

impl JavaScript {
    pub fn from(path: impl Into<String>) -> Self {
        JavaScript {
            source: Source::From(path.into()),
            ..Default::default()
        }
    }

    pub fn defer(path: impl Into<String>) -> Self {
        JavaScript {
            source: Source::Defer(path.into()),
            ..Default::default()
        }
    }

    pub fn asynchronous(path: impl Into<String>) -> Self {
        JavaScript {
            source: Source::Async(path.into()),
            ..Default::default()
        }
    }

    pub fn inline(name: impl Into<String>, script: impl Into<String>) -> Self {
        JavaScript {
            source: Source::Inline(name.into(), script.into()),
            ..Default::default()
        }
    }

    pub fn on_load(name: impl Into<String>, script: impl Into<String>) -> Self {
        JavaScript {
            source: Source::OnLoad(name.into(), script.into()),
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
}
