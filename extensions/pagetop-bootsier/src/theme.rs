//! Definiciones y plantillas del tema Bootsier.

pub mod bs;

pub mod class;

mod token;
pub use token::*;

#[doc(hidden)]
pub use bs::badge::BadgeBootsier;
#[doc(hidden)]
pub use bs::container::ContainerBootsier;
#[doc(hidden)]
pub use bs::form::input::InputBootsier;
#[doc(hidden)]
pub use bs::form::select::SelectBootsier;
#[doc(hidden)]
pub use bs::form::textarea::TextareaBootsier;

// Image.
pub mod image;
#[doc(inline)]
pub use image::Image;

// Nav.
pub mod nav;
#[doc(inline)]
pub use nav::Nav;

// Navbar.
pub mod navbar;
#[doc(inline)]
pub use navbar::Navbar;

// Offcanvas.
pub mod offcanvas;
#[doc(inline)]
pub use offcanvas::Offcanvas;
