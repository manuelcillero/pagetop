//! Respuestas a las peticiones web en sus diferentes formatos.

pub use actix_web::ResponseError;

pub mod page;

pub mod json;

pub mod redirect;
