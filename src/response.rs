//! Respuestas a las peticiones web en sus diferentes formatos.

mod page;
pub use page::*;

mod json;
pub use json::*;

mod redirect;
pub use redirect::*;

mod flash;
pub use flash::ResponseExt;
pub(crate) use flash::{keep_flash_messages, read_flash_messages};

mod waypoint;
pub use waypoint::*;
