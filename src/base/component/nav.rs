//! Definiciones para crear menús de navegación planos ([`Nav`]) y sus elementos ([`Item`]).

mod props;
pub use props::Layout;

mod component;
pub use component::Nav;

mod item;
pub use item::{Item, ItemKind};
