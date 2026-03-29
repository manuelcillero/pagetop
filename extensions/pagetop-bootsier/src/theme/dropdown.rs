//! Definiciones para crear menús desplegables [`Dropdown`].
//!
//! Cada [`dropdown::Item`](crate::theme::dropdown::Item) representa un elemento individual del
//! desplegable [`Dropdown`], con distintos comportamientos según su finalidad, como enlaces de
//! navegación, botones de acción, encabezados o divisores visuales.
//!
//! Los ítems pueden estar activos, deshabilitados o abrirse en nueva ventana según su contexto y
//! configuración, y permiten incluir etiquetas localizables usando [`L10n`](pagetop::locale::L10n).
//!
//! # Ejemplo
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let dd = Dropdown::new()
//!     .with_title(L10n::n("Menu"))
//!     .with_button_color(ButtonColor::Background(Color::Secondary))
//!     .with_auto_close(dropdown::AutoClose::ClickableInside)
//!     .with_direction(dropdown::Direction::Dropend)
//!     .with_item(dropdown::Item::link(L10n::n("Home"), |_| "/".into()))
//!     .with_item(dropdown::Item::link_blank(L10n::n("External"), |_| "https://docs.rs".into()))
//!     .with_item(dropdown::Item::divider())
//!     .with_item(dropdown::Item::header(L10n::n("User session")))
//!     .with_item(dropdown::Item::button(L10n::n("Sign out")));
//! ```

mod props;
pub use props::{AutoClose, Direction, MenuAlign, MenuPosition};

mod component;
pub use component::Dropdown;

mod item;
pub use item::{Item, ItemKind};
