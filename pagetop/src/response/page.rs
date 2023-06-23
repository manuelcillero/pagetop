pub use actix_web::Result as ResultPage;

mod before_prepare_page;
pub use before_prepare_page::{ActionBeforePreparePage, ACTION_BEFORE_PREPARE_PAGE};

mod after_prepare_page;
pub use after_prepare_page::{ActionAfterPreparePage, ACTION_AFTER_PREPARE_PAGE};

mod definition;
pub use definition::Page;
