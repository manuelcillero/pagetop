//! Definiciones para crear barras de navegación [`Navbar`].
//!
//! Cada [`navbar::Item`](crate::theme::navbar::Item) representa un elemento individual de la barra
//! de navegación [`Navbar`], con distintos comportamientos según su finalidad, como menús
//! [`Nav`](crate::theme::Nav) o *textos localizados* usando [`L10n`](pagetop::locale::L10n).
//!
//! También puede mostrar una marca de identidad ([`navbar::Brand`](crate::theme::navbar::Brand))
//! que identifique la compañía, producto o nombre del proyecto asociado a la solución web.
//!
//! # Ejemplos
//!
//! Barra **simple**, sólo con un menú horizontal:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let navbar = Navbar::simple()
//!     .add_item(navbar::Item::nav(
//!         Nav::new()
//!             .add_item(nav::Item::link(L10n::n("Home"), |_| "/".into()))
//!             .add_item(nav::Item::link(L10n::n("About"), |_| "/about".into()))
//!             .add_item(nav::Item::link(L10n::n("Contact"), |_| "/contact".into()))
//!     ));
//! ```
//!
//! Barra **colapsable**, con botón de despliegue y contenido en el desplegable cuando colapsa:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let navbar = Navbar::simple_toggle()
//!     .with_expand(BreakPoint::MD)
//!     .add_item(navbar::Item::nav(
//!         Nav::new()
//!             .add_item(nav::Item::link(L10n::n("Home"), |_| "/".into()))
//!             .add_item(nav::Item::link_blank(L10n::n("Docs"), |_| "https://sample.com".into()))
//!             .add_item(nav::Item::link(L10n::n("Support"), |_| "/support".into()))
//!     ));
//! ```
//!
//! Barra con **marca de identidad a la izquierda** y menú a la derecha, típica de una cabecera:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let brand = navbar::Brand::new()
//!     .with_title(L10n::n("PageTop"))
//!     .with_path(Some(|_| "/".into()));
//!
//! let navbar = Navbar::brand_left(brand)
//!     .add_item(navbar::Item::nav(
//!         Nav::new()
//!             .add_item(nav::Item::link(L10n::n("Home"), |_| "/".into()))
//!             .add_item(nav::Item::dropdown(
//!                 Dropdown::new()
//!                     .with_title(L10n::n("Tools"))
//!                     .add_item(dropdown::Item::link(
//!                         L10n::n("Generator"), |_| "/tools/gen".into())
//!                     )
//!                     .add_item(dropdown::Item::link(
//!                         L10n::n("Reports"), |_| "/tools/reports".into())
//!                     )
//!             ))
//!             .add_item(nav::Item::link_disabled(L10n::n("Disabled"), |_| "#".into()))
//!     ));
//! ```
//!
//! Barra con **botón de despliegue a la izquierda** y **marca de identidad a la derecha**:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let brand = navbar::Brand::new()
//!     .with_title(L10n::n("Intranet"))
//!     .with_path(Some(|_| "/".into()));
//!
//! let navbar = Navbar::brand_right(brand)
//!     .with_expand(BreakPoint::LG)
//!     .add_item(navbar::Item::nav(
//!         Nav::pills()
//!             .add_item(nav::Item::link(L10n::n("Dashboard"), |_| "/dashboard".into()))
//!             .add_item(nav::Item::link(L10n::n("Users"), |_| "/users".into()))
//!     ));
//! ```
//!
//! Barra con el **contenido en un *offcanvas***, ideal para dispositivos móviles o menús largos:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let oc = Offcanvas::new()
//!     .with_id("main_offcanvas")
//!     .with_title(L10n::n("Main menu"))
//!     .with_placement(offcanvas::Placement::Start)
//!     .with_backdrop(offcanvas::Backdrop::Enabled);
//!
//! let navbar = Navbar::offcanvas(oc)
//!     .add_item(navbar::Item::nav(
//!         Nav::new()
//!             .add_item(nav::Item::link(L10n::n("Home"), |_| "/".into()))
//!             .add_item(nav::Item::link(L10n::n("Profile"), |_| "/profile".into()))
//!             .add_item(nav::Item::dropdown(
//!                 Dropdown::new()
//!                     .with_title(L10n::n("More"))
//!                     .add_item(dropdown::Item::link(L10n::n("Settings"), |_| "/settings".into()))
//!                     .add_item(dropdown::Item::link(L10n::n("Help"), |_| "/help".into()))
//!             ))
//!     ));
//! ```
//!
//! Barra **fija arriba**:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let brand = navbar::Brand::new()
//!     .with_title(L10n::n("Main App"))
//!     .with_path(Some(|_| "/".into()));
//!
//! let navbar = Navbar::brand_left(brand)
//!     .with_position(navbar::Position::FixedTop)
//!     .add_item(navbar::Item::nav(
//!         Nav::new()
//!             .add_item(nav::Item::link(L10n::n("Dashboard"), |_| "/".into()))
//!             .add_item(nav::Item::link(L10n::n("Donors"), |_| "/donors".into()))
//!             .add_item(nav::Item::link(L10n::n("Stock"), |_| "/stock".into()))
//!     ));
//! ```

mod props;
pub use props::{Layout, Position};

mod brand;
pub use brand::Brand;

mod component;
pub use component::Navbar;

mod item;
pub use item::Item;
