//! Definiciones para crear menús desplegables [`Dropdown`].
//!
//! Cada [`dropdown::Item`](crate::theme::dropdown::Item) representa un elemento individual del
//! desplegable [`Dropdown`], con distintos comportamientos según su finalidad, como enlaces de
//! navegación, botones de acción, encabezados o divisores visuales.
//!
//! Los ítems pueden estar activos, deshabilitados o abrirse en nueva ventana según su contexto y
//! configuración, y permiten incluir etiquetas localizables usando [`L10n`](pagetop::locale::L10n).

mod props;
pub use props::{AutoClose, Direction, MenuAlign, MenuPosition};

mod component;
pub use component::Dropdown;

mod item;
pub use item::{Item, ItemKind};
