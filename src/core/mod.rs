pub use actix_web::dev::Server;

mod state;
pub use state::register_theme;
pub use state::register_module;
pub use state::add_component_to;

mod all;

pub mod theme;
pub mod module;
pub mod response;
pub mod server;
