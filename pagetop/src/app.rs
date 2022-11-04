pub use actix_web::{
    http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
pub use actix_web_files::Files as ActixFiles;
pub use actix_web_static_files::ResourceFiles;

pub mod config;
pub use config::SETTINGS;

mod banner;

mod tracing;

pub mod locale;

#[cfg(feature = "database")]
pub mod db;

pub mod application;

pub mod fatal_error;
