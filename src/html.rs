//! HTML en código.

mod maud;
pub use maud::{display, html, html_private, Escaper, Markup, PreEscaped, DOCTYPE};

mod route;
pub use route::RoutePath;

// **< HTML DOCUMENT ASSETS >***********************************************************************

mod assets;
pub use assets::favicon::Favicon;
pub use assets::javascript::JavaScript;
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub use assets::{Asset, Assets};

mod logo;
pub use logo::PageTopSvg;

// **< HTML ATTRIBUTES >****************************************************************************

mod attr;
pub use attr::{Attr, AttrId, AttrName, AttrValue};

mod classes;
pub use classes::{Classes, ClassesOp};

mod unit;
pub use unit::UnitValue;
