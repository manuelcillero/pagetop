use super::AssetsTrait;
use crate::html::{html, Markup};

#[rustfmt::skip]
#[derive(Default)]
pub struct HeadStyles {
    path  : String,
    styles: String,
    weight: isize,
}

impl AssetsTrait for HeadStyles {
    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn weight(&self) -> isize {
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

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }
}
