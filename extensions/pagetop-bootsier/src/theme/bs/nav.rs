//! Definiciones para crear menús ([`Nav`]).
//!
//! Cada [`nav::Item`](crate::theme::bs::nav::Item) representa un elemento individual del menú
//! [`Nav`], con distintos comportamientos según su finalidad, como enlaces de navegación o menús
//! desplegables [`Dropdown`](crate::theme::bs::Dropdown).
//!
//! Los ítems pueden estar activos, deshabilitados o abrirse en nueva ventana según su contexto y
//! configuración, y permiten incluir etiquetas localizables usando [`Lc`](pagetop::locale::Lc).

mod props;
pub use props::{Kind, Layout};

mod component;
pub use component::Nav;

mod item;
pub use item::{Item, ItemKind};
