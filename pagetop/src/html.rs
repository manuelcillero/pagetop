pub use maud::{html, Markup, PreEscaped, DOCTYPE};

mod assets;
pub use assets::javascript::{JavaScript, ModeJS};
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub use assets::Assets;

mod favicon;
pub use favicon::Favicon;

mod attribute;
pub use attribute::AttributeValue;

mod identifier;
pub use identifier::IdentifierValue;

mod classes;
pub use classes::{Classes, ClassesOp};
