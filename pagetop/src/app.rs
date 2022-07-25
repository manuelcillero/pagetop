pub use actix_web::{http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
pub use actix_web_static_files::ResourceFiles;
pub use actix_files::Files as ActixFiles;

mod banner;

mod tracing;

pub mod locale;

#[cfg(feature = "database")]
pub mod db;

mod definition;
pub use definition::AppTrait;

pub mod application;

pub mod fatal_error;