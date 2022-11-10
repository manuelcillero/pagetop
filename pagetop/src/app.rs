pub use actix_web::{
    http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
pub use actix_web_files::Files as ActixFiles;
pub use actix_web_static_files::ResourceFiles;

mod banner;

pub mod application;

pub mod fatal_error;
