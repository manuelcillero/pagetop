use crate::response::page::Page;

pub type ActionPage = fn(page: &mut Page);

mod before_prepare_page;
pub use before_prepare_page::*;

mod before_render_page;
pub use before_render_page::*;
