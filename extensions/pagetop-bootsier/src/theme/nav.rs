//! Definiciones para crear menús [`Nav`] o alguna de sus variantes de presentación.
//!
//! Cada [`nav::Item`](crate::theme::nav::Item) representa un elemento individual del menú [`Nav`],
//! con distintos comportamientos según su finalidad, como enlaces de navegación o menús
//! desplegables [`Dropdown`](crate::theme::Dropdown).
//!
//! Los ítems pueden estar activos, deshabilitados o abrirse en nueva ventana según su contexto y
//! configuración, y permiten incluir etiquetas localizables usando [`L10n`](pagetop::locale::L10n).

mod props;
pub use props::{Kind, Layout};

mod component;
pub use component::Nav;

mod item;
pub use item::{Item, ItemKind};
