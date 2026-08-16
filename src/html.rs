//! HTML en código.

pub(crate) mod maud;
pub use maud::DOCTYPE;
pub use maud::{Escaper, Markup, PreEscaped, Render, RenderAttrs};
pub use maud::{display, html, html_private};

mod route_path;
pub use route_path::RoutePath;

mod sort_dir;
pub use sort_dir::SortDir;

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
pub use props::{Props, PropsError, PropsExtra, PropsOp};

mod unit;
pub use unit::UnitValue;
