use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};
use crate::{SmartDefault, Weight};

#[rustfmt::skip]
#[derive(SmartDefault)]
pub struct HeadScript {
    path  : String,
    code  : String,
    weight: Weight,
}

impl AssetsTrait for HeadScript {
    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn prepare(&self) -> Markup {
        html! { script { (self.code) }; }
    }
}

impl HeadScript {
    pub fn named(path: impl Into<String>) -> Self {
        HeadScript {
            path: path.into(),
            ..Default::default()
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into().trim().to_owned();
        self
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}
