pub use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, http, web
};
use actix_web::dev::Server;

mod banner;

mod tracing;

pub mod locale;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub mod db;

pub mod application;
