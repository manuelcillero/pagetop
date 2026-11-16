//! Definiciones para la composición de documentos ([`Region`], [`Template`] y [`AdminFrame`]).

mod admin_frame;
pub use admin_frame::AdminFrame;

mod region;
pub use region::Region;

mod template;
pub use template::Template;
