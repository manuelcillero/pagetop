//! Definiciones para crear menús desplegables [`Dropdown`].
//!
//! Cada [`dropdown::Item`](crate::theme::dropdown::Item) representa un elemento individual del
//! desplegable [`Dropdown`], con distintos comportamientos según su finalidad: enlaces de
//! navegación, botones de acción, encabezados o divisores visuales.
//!
//! Los ítems pueden estar activos, deshabilitados o abrirse en nueva ventana según su contexto y
//! configuración, y permiten incluir etiquetas localizables usando [`L10n`](pagetop::locale::L10n).
//!
//! Su propósito es ofrecer una base uniforme sobre la que construir menús consistentes, adaptados
//! al contexto de cada aplicación.

mod component;
pub use component::Dropdown;
pub use component::{AutoClose, Direction, MenuAlign, MenuPosition};

mod item;
pub use item::{Item, ItemKind};
