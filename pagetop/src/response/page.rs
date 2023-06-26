pub use actix_web::Result as ResultPage;

mod before_prepare_page;
pub use before_prepare_page::{ActionBeforePreparePage, ACTION_BEFORE_PREPARE_PAGE};

mod before_render_page;
pub use before_render_page::{ActionBeforeRenderPage, ACTION_BEFORE_RENDER_PAGE};

mod definition;
pub use definition::Page;
