pub use actix_web::dev::Server;

mod state;
pub use state::register_module;

mod all;

pub mod module;
pub mod server;
