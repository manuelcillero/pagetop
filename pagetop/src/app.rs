pub use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

mod banner;

mod tracing;

pub mod locale;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub mod db;

mod definition;
pub use definition::AppTrait;

pub mod application;
