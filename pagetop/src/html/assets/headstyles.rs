use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};
use crate::Weight;

#[rustfmt::skip]
#[derive(Default)]
pub struct HeadStyles {
    path  : String,
    styles: String,
    weight: Weight,
}

impl AssetsTrait for HeadStyles {
    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn prepare(&self) -> Markup {
        html! { styles { (self.styles) }; }
    }
}

impl HeadStyles {
    pub fn named<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        HeadStyles {
            path: path.into(),
            ..Default::default()
        }
    }

    pub fn with_styles(mut self, styles: &str) -> Self {
        self.styles = styles.trim().to_owned();
        self
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}
