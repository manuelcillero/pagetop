pub use maud::{DOCTYPE, Markup, PreEscaped, html};

mod assets;
pub use assets::{Assets, AssetsOp, SourceValue};
pub use assets::javascript::{JavaScript, JSMode};
pub use assets::stylesheet::StyleSheet;

mod favicon;
pub use favicon::Favicon;

mod attribute;
pub use attribute::AttributeValue;

mod identifier;
pub use identifier::IdentifierValue;

mod classes;
pub use classes::{Classes, ClassesOp, ClassValue};

mod unit;
pub use unit::UnitValue;

mod layout;
pub use layout::{InlineLayout, LayoutSet};
