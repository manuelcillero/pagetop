//! Componentes proporcionados por el tema.

pub(crate) mod layout;
pub use layout::BootsierRegions;

// Badge.
pub(crate) mod badge;
pub use badge::{Badge, BadgeBootsier};

// Block.
pub use pagetop::base::component::Block;

// Brand.
pub(crate) mod brand;
pub use brand::Brand;

// Breadcrumb.
#[doc(inline)]
pub use breadcrumb::Breadcrumb;
pub use pagetop::base::component::breadcrumb;

// Button.
pub mod button;
pub use button::{Button, ButtonBootsier};

// Container.
pub mod container;
#[doc(inline)]
pub use container::Container;
#[doc(inline)]
pub use container::ContainerBootsier;

// Dialog.
pub mod dialog;
#[doc(inline)]
pub use dialog::Dialog;

// Dropdown.
pub mod dropdown;
#[doc(inline)]
pub use dropdown::Dropdown;
#[doc(inline)]
pub use dropdown::DropdownBootsier;

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

// Messages.
pub use pagetop::base::component::Messages;

// Nav.
pub mod nav;
#[doc(inline)]
pub use nav::Nav;
#[doc(inline)]
pub use nav::NavBootsier;

// Navbar.
pub mod navbar;
#[doc(inline)]
pub use navbar::Navbar;
#[doc(inline)]
pub use navbar::NavbarBootsier;

// Offcanvas.
pub mod offcanvas;
#[doc(inline)]
pub use offcanvas::Offcanvas;

// Pager.
pub use pagetop::base::component::{Pager, PagerAlign, PagerVisibility};

// Sidebar (componentes de navegación de AdminLTE).
pub mod sidebar;

// Table.
pub use pagetop::base::component::table;
#[doc(inline)]
pub use table::Table;
