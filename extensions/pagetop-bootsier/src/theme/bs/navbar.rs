//! Definiciones para crear barras de navegación ([`Navbar`]).
//!
//! Cada [`navbar::Item`](crate::theme::bs::navbar::Item) representa un elemento individual de la
//! barra de navegación [`Navbar`], con distintos comportamientos según su finalidad, como menús
//! [`Nav`](crate::theme::bs::Nav) o *textos localizados* usando [`Lc`](pagetop::locale::Lc).
//!
//! También puede añadir una marca de identidad ([`navbar::Brand`](crate::theme::bs::navbar::Brand))
//! que identifique la compañía, producto o nombre del proyecto asociado a la solución web.

mod props;
pub use props::{Layout, Position};

pub use super::Brand;

pub use pagetop::base::component::Navbar;
pub use pagetop::base::component::navbar::Item;

mod component;
pub use component::NavbarBootsier;
pub(crate) use component::{render, setup};

mod item;
pub(crate) use item::render as item_render;
