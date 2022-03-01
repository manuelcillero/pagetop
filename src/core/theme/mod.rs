pub use maud::{DOCTYPE, Markup, PreEscaped, html};

mod api;
pub use api::{Theme, find_theme, register_theme};
