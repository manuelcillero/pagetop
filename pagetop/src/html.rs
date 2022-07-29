pub use maud::{html, Markup, PreEscaped, DOCTYPE};

mod assets;
pub use assets::javascript::{JSMode, JavaScript};
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub use assets::Assets;

mod favicon;
pub use favicon::Favicon;

mod attribute;
pub use attribute::AttributeValue;

mod identifier;
pub use identifier::IdentifierValue;

mod classes;
pub use classes::{ClassValue, Classes, ClassesOp};
