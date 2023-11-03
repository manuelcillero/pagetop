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

mod opt_translated;
pub use opt_translated::OptionTranslated;

mod opt_classes;
pub use opt_classes::{ClassesOp, OptionClasses};

pub mod unit;

pub enum PrepareMarkup {
    None,
    Text(&'static str),
    With(Markup),
}
