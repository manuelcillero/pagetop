//! Tipos y funciones para operar con el servidor web ([actix-web](https://docs.rs/actix-web)).

pub use actix_session::Session;
pub use actix_web::{
    cookie, http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};

pub use actix_web_files::Files as ActixFiles;
pub use actix_web_static_files::ResourceFiles;
