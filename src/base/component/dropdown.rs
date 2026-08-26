//! Definiciones para crear menús desplegables ([`Dropdown`]) y sus elementos ([`Item`]).

mod component;
pub use component::Dropdown;

mod item;
pub use item::{Item, ItemKind};
