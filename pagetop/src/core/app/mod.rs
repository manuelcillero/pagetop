pub use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, http, web
};

mod banner;

mod tracing;

pub mod locale;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub mod db;

mod definition;
pub use definition::AppTrait;

pub mod application;
