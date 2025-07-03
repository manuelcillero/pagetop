//! Gestión del servidor y servicios web ([actix-web](https://docs.rs/actix-web)).

pub use actix_web::body::BoxBody;
pub use actix_web::dev::Server;
pub use actix_web::dev::ServiceFactory as Factory;
pub use actix_web::dev::ServiceRequest as Request;
pub use actix_web::dev::ServiceResponse as Response;
pub use actix_web::{http, rt, test};
pub use actix_web::{App, Error, HttpServer};
