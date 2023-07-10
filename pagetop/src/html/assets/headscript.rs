use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};

#[rustfmt::skip]
#[derive(Default)]
pub struct HeadScript {
    path  : String,
    code  : String,
    weight: isize,
}

impl AssetsTrait for HeadScript {
    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn weight(&self) -> isize {
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

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }
}
