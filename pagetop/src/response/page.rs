pub use actix_web::Result as ResultPage;

mod hook;
pub use hook::{BeforeRenderPageHook, HOOK_BEFORE_RENDER_PAGE};

mod definition;
pub use definition::Page;
