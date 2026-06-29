//! Definiciones para crear contenedores de componentes ([`Container`]).
//!
//! Cada contenedor envuelve contenido usando la etiqueta semántica indicada por
//! [`container::Kind`](crate::theme::bs::container::Kind).
//!
//! Con [`container::Width`](crate::theme::bs::container::Width) se puede definir el ancho y el
//! comportamiento *responsive* del contenedor. También permite aplicar utilidades de estilo para el
//! fondo, texto, borde o esquinas redondeadas.

mod props;
pub use props::{Kind, Width};

mod component;
pub use component::Container;
