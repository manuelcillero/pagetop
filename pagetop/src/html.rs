pub use maud::{DOCTYPE, Markup, PreEscaped, html};

mod assets;
pub use assets::Assets;
pub use assets::javascript::{JavaScript, JSMode};
pub use assets::stylesheet::StyleSheet;

mod favicon;
pub use favicon::Favicon;

mod attribute;
pub use attribute::AttributeValue;

mod identifier;
pub use identifier::IdentifierValue;

mod classes;
pub use classes::{Classes, ClassesOp};

mod spacing;
pub use spacing::{Spaces, SpaceSet, SpaceValue};
