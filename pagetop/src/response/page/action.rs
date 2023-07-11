use crate::response::page::Page;

pub type ActionPage = fn(page: &mut Page);

mod before_prepare_body;
pub use before_prepare_body::*;

mod after_prepare_body;
pub use after_prepare_body::*;
