//! Definiciones y componentes del tema.
//!
//! En esta página, el apartado **Modules** incluye las definiciones necesarias para los componentes
//! que se muestran en el apartado **Structs**, mientras que en **Enums** se listan los elementos
//! auxiliares del tema utilizados en clases y componentes.

mod aux;
pub use aux::*;

pub mod classes;

// Container.
pub mod container;
#[doc(inline)]
pub use container::Container;

// Dropdown.
pub mod dropdown;
#[doc(inline)]
pub use dropdown::Dropdown;

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
