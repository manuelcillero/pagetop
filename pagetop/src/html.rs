//! HTML en código.

mod maud;
pub use maud::{html, html_private, Markup, PreEscaped, DOCTYPE};

mod assets;
pub use assets::headscript::HeadScript;
pub use assets::headstyles::HeadStyles;
pub use assets::javascript::{JavaScript, ModeJS};
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub use assets::Assets;

mod favicon;
pub use favicon::Favicon;

mod opt_id;
pub use opt_id::OptionId;

mod opt_name;
pub use opt_name::OptionName;

mod opt_string;
pub use opt_string::OptionString;

mod opt_classes;
pub use opt_classes::{ClassesOp, OptionClasses};

pub mod unit;

pub enum PrepareMarkup {
    None,
    Text(&'static str),
    With(Markup),
}

impl PrepareMarkup {
    pub fn html(self) -> Markup {
        match self {
            PrepareMarkup::None => html! {},
            PrepareMarkup::Text(text) => html! { (text) },
            PrepareMarkup::With(markup) => markup,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            PrepareMarkup::None => None,
            PrepareMarkup::Text(text) => {
                if text.is_empty() {
                    None
                } else {
                    Some(text.to_string())
                }
            }
            PrepareMarkup::With(markup) => Some(markup.into_string()),
        }
    }
}

impl ToString for PrepareMarkup {
    fn to_string(&self) -> String {
        match self {
            PrepareMarkup::None => "".to_owned(),
            PrepareMarkup::Text(text) => text.to_string(),
            PrepareMarkup::With(markup) => markup.to_owned().into_string(),
        }
    }
}
