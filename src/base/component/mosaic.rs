//! Definiciones para repartir bloques en un mosaico de columnas ([`Mosaic`]) y para los bloques con
//! ajustes propios ([`Item`]), como las columnas que ocupan ([`Span`]).

mod component;
pub use component::Mosaic;

mod item;
pub use item::{Item, Span};
