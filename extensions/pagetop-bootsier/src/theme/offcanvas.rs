//! Definiciones para crear paneles laterales deslizantes [`Offcanvas`].
//!
//! # Ejemplo
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # use pagetop_bootsier::prelude::*;
//! let panel = Offcanvas::new()
//!     .with_id("offcanvas_example")
//!     .with_title(L10n::n("Offcanvas title"))
//!     .with_placement(offcanvas::Placement::End)
//!     .with_backdrop(offcanvas::Backdrop::Enabled)
//!     .with_body_scroll(offcanvas::BodyScroll::Enabled)
//!     .with_visibility(offcanvas::Visibility::Default)
//!     .add_child(Dropdown::new()
//!         .with_title(L10n::n("Menu"))
//!         .add_item(dropdown::Item::label(L10n::n("Label")))
//!         .add_item(dropdown::Item::link_blank(L10n::n("Google"), |_| "https://www.google.es"))
//!         .add_item(dropdown::Item::link(L10n::n("Sign out"), |_| "/signout"))
//!     );
//! ```

mod props;
pub use props::{Backdrop, BodyScroll, Placement, Visibility};

mod component;
pub use component::Offcanvas;
