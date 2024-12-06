//! Respuestas a las peticiones web en sus diferentes variantes.

pub use actix_web::ResponseError;

pub mod page;

pub mod json;

pub mod redirect;
