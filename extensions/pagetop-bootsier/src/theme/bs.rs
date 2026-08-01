//! Componentes proporcionados por el tema.

// Badge.
pub(crate) mod badge;
pub use badge::{Badge, BadgeBootsier};

// Button.
mod button;
pub use button::{Button, ButtonAction};

// Container.
pub mod container;
#[doc(inline)]
pub use container::Container;
#[doc(inline)]
pub use container::ContainerBootsier;

// Dropdown.
pub mod dropdown;
#[doc(inline)]
pub use dropdown::Dropdown;

// Form.
pub mod form;
#[doc(inline)]
pub use form::Form;
#[doc(inline)]
pub use form::input::InputBootsier;
#[doc(inline)]
pub use form::select::SelectBootsier;
#[doc(inline)]
pub use form::textarea::TextareaBootsier;

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

// Sidebar (componentes de navegación de AdminLTE).
pub mod sidebar;
