pub use actix_web::Result as ResultPage;

mod action;
pub use action::{ActionBeforeRenderPage, ACTION_BEFORE_RENDER_PAGE};

mod definition;
pub use definition::Page;
