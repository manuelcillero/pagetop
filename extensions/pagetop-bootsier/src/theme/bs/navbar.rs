//! Definiciones para crear barras de navegación ([`Navbar`]).
//!
//! Cada [`navbar::Item`](crate::theme::bs::navbar::Item) representa un elemento individual de la
//! barra de navegación [`Navbar`], con distintos comportamientos según su finalidad, como menús
//! [`Nav`](crate::theme::bs::Nav) o *textos localizados* usando [`L10n`](pagetop::locale::L10n).
//!
//! También puede añadir una marca de identidad ([`navbar::Brand`](crate::theme::bs::navbar::Brand))
//! que identifique la compañía, producto o nombre del proyecto asociado a la solución web.

mod props;
pub use props::{Layout, Position};

mod brand;
pub use brand::Brand;

mod component;
pub use component::Navbar;

mod item;
pub use item::Item;
