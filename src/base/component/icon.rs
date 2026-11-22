//! Definiciones para mostrar iconos ([`Icon`]).
//!
//! Un icono se construye a partir de un [`icon::Source`](Source): un SVG en línea, un icono del
//! pequeño conjunto que incluye PageTop ([`icon::Core`](Core)) o un glifo de una fuente de iconos.
//! Cualquier otro conjunto se integra implementando `From<T> for icon::Source` para su propio tipo.

mod source;
pub use source::Source;

mod core_set;
pub use core_set::Core;

mod component;
pub use component::Icon;
