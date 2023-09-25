use crate::prelude::*;

pub type FnActionPage = fn(page: &mut Page);

mod before_prepare_body;
pub use before_prepare_body::*;

mod after_prepare_body;
pub use after_prepare_body::*;
