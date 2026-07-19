//! Respuestas a las peticiones web en sus diferentes formatos.

mod page;
pub use page::*;

mod json;
pub use json::*;

mod redirect;
pub use redirect::*;

mod waypoint;
pub use waypoint::*;
