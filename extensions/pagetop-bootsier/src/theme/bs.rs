//! Tipos y componentes disponibles.
//!
//! A continuación, el apartado **Modules** incluye las definiciones necesarias para los componentes
//! que se muestran en el apartado **Structs**, mientras que en **Enums** se listan los elementos
//! auxiliares del tema utilizados en clases y componentes.

mod attrs;
pub use attrs::*;

mod classes;
pub use classes::*;

// Button.
mod button;
pub use button::{Button, ButtonAction};

// Container.
pub mod container;
#[doc(inline)]
pub use container::Container;

// Dropdown.
pub mod dropdown;
#[doc(inline)]
pub use dropdown::Dropdown;

// Form.
pub mod form;
#[doc(inline)]
pub use form::Form;

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
