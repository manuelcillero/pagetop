use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};
use crate::Weight;

#[rustfmt::skip]
#[derive(Default)]
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
    pub fn named<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        HeadScript {
            path: path.into(),
            ..Default::default()
        }
    }

    pub fn with_code(mut self, code: &str) -> Self {
        self.code = code.trim().to_owned();
        self
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}
