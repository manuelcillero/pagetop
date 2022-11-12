pub use actix_web::ResponseError;

pub mod page;

mod fatal_error;
pub use fatal_error::FatalError;
