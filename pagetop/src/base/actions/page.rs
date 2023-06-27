use crate::prelude::*;

pub type ActionPage = fn(page: &mut Page);

mod before_prepare_page;
pub use before_prepare_page::{
    run_actions_before_prepare_page, ActionBeforePreparePage, ACTION_BEFORE_PREPARE_PAGE,
};

mod before_render_page;
pub use before_render_page::{
    run_actions_before_render_page, ActionBeforeRenderPage, ACTION_BEFORE_RENDER_PAGE,
};
