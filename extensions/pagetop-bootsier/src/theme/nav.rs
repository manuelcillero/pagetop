//! Definiciones para crear menús [`Nav`] o alguna de sus variantes de presentación.
//!
//! Cada [`nav::Item`](crate::theme::nav::Item) representa un elemento individual del menú [`Nav`],
//! con distintos comportamientos según su finalidad, como enlaces de navegación o menús
//! desplegables [`Dropdown`](crate::theme::Dropdown).
//!
//! Los ítems pueden estar activos, deshabilitados o abrirse en nueva ventana según su contexto y
//! configuración, y permiten incluir etiquetas localizables usando [`L10n`](pagetop::locale::L10n).
//!
//! # Ejemplo
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let nav = Nav::tabs()
//!     .with_layout(nav::Layout::End)
//!     .add_item(nav::Item::link(L10n::n("Home"), |_| "/"))
//!     .add_item(nav::Item::link_blank(L10n::n("External"), |_| "https://www.google.es"))
//!     .add_item(nav::Item::dropdown(
//!         Dropdown::new()
//!             .with_title(L10n::n("Options"))
//!             .with_items(TypedOp::AddMany(vec![
//!                 Typed::with(dropdown::Item::link(L10n::n("Action"), |_| "/action")),
//!                 Typed::with(dropdown::Item::link(L10n::n("Another action"), |_| "/another")),
//!             ])),
//!     ))
//!     .add_item(nav::Item::link_disabled(L10n::n("Disabled"), |_| "#"));
//! ```

mod props;
pub use props::{Kind, Layout};

mod component;
pub use component::Nav;

mod item;
pub use item::{Item, ItemKind};
