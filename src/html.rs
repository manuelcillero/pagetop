//! HTML en código.

pub(crate) mod maud;
pub use maud::{DOCTYPE, Escaper, Markup, PreEscaped, Render, display, html, html_private};

mod route;
pub use route::RoutePath;

// **< HTML DOCUMENT ASSETS >***********************************************************************

mod assets;
pub use assets::favicon::Favicon;
pub use assets::javascript::JavaScript;
pub use assets::preload::Preload;
pub use assets::stylesheet::{StyleSheet, TargetMedia};
pub use assets::{Asset, Assets};

mod logo;
pub use logo::PageTopSvg;

// **< HTML ATTRIBUTES >****************************************************************************

mod attr;
pub use attr::{Attr, AttrName, AttrValue};

mod props;
pub use props::{Props, PropsOp};

mod unit;
pub use unit::UnitValue;
