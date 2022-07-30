use crate::html::{Favicon, StyleSheet, JavaScript};

pub use actix_web::Result as ResultPage;

pub enum PageOp {
    SetTheme(&'static str),

    AddFavicon(Favicon),
    RemoveFavicon,

    AddMetadata(&'static str, &'static str),
    AddProperty(&'static str, &'static str),

    AddStyleSheet(StyleSheet),
    RemoveStyleSheet(&'static str),

    AddJavaScript(JavaScript),
    RemoveJavaScript(&'static str),
    AddJQuery,
}

mod context;
pub use context::PageContext;

mod hook;
pub use hook::{BeforeRenderPageHook, HOOK_BEFORE_RENDER_PAGE};

mod definition;
pub use definition::Page;
