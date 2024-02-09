use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};
use crate::{SmartDefault, Weight};

#[derive(Default, Eq, PartialEq)]
pub enum ModeJS {
    Async,
    #[default]
    Defer,
    Normal,
}

#[rustfmt::skip]
#[derive(SmartDefault)]
pub struct JavaScript {
    path   : String,
    prefix : &'static str,
    version: &'static str,
    weight : Weight,
    mode   : ModeJS,
}

impl AssetsTrait for JavaScript {
    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn prepare(&self) -> Markup {
        html! {
            script type="text/javascript"
                src=(crate::concat_string!(self.path, self.prefix, self.version))
                async[self.mode == ModeJS::Async]
                defer[self.mode == ModeJS::Defer]
                {};
        }
    }
}

impl JavaScript {
    pub fn at(path: impl Into<String>) -> Self {
        JavaScript {
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

    pub fn with_mode(mut self, mode: ModeJS) -> Self {
        self.mode = mode;
        self
    }
}
