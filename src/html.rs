//! HTML in code.

mod maud;
pub use maud::{html, html_private, Markup, PreEscaped, DOCTYPE};

mod assets;
pub use assets::favicon::Favicon;
pub use assets::javascript::JavaScript;
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub(crate) use assets::Assets;

mod opt_id;
pub use opt_id::OptionId;

mod opt_name;
pub use opt_name::OptionName;

mod opt_string;
pub use opt_string::OptionString;

mod opt_translated;
pub use opt_translated::OptionTranslated;

mod opt_classes;
pub use opt_classes::{ClassesOp, OptionClasses};

mod opt_component;
pub use opt_component::OptionComponent;

pub mod unit;

pub enum PrepareMarkup {
    None,
    Text(&'static str),
    With(Markup),
}

impl PrepareMarkup {
    pub fn render(&self) -> Markup {
        match self {
            PrepareMarkup::None => html! {},
            PrepareMarkup::Text(text) => html! { (text) },
            PrepareMarkup::With(markup) => html! { (markup) },
        }
    }
}
