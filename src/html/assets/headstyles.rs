use crate::html::assets::AssetsTrait;
use crate::html::{html, Markup};
use crate::{SmartDefault, Weight};

#[rustfmt::skip]
#[derive(SmartDefault)]
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
    pub fn named(path: impl Into<String>) -> Self {
        HeadStyles {
            path: path.into(),
            ..Default::default()
        }
    }

    pub fn with_styles(mut self, styles: impl Into<String>) -> Self {
        self.styles = styles.into().trim().to_owned();
        self
    }

    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}
