//! HTML en código.

mod maud;
pub use maud::{html, html_private, Markup, PreEscaped, DOCTYPE};

mod assets;
pub use assets::javascript::{JavaScript, ModeJS};
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub use assets::Assets;

mod favicon;
pub use favicon::Favicon;

mod identifier;
pub use identifier::IdentifierValue;

mod name;
pub use name::NameValue;

mod attribute;
pub use attribute::AttributeValue;

mod classes;
pub use classes::{Classes, ClassesOp};
