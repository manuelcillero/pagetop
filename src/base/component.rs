//! Componentes nativos proporcionados por PageTop.

pub mod layout;

mod badge;
pub use badge::Badge;

mod brand;
pub use brand::Brand;

pub mod breadcrumb;
#[doc(inline)]
pub use breadcrumb::Breadcrumb;

mod block;
pub use block::Block;

pub mod button;
#[doc(inline)]
pub use button::{Button, ButtonSet};

pub mod container;
#[doc(inline)]
pub use container::Container;

mod dialog;
pub use dialog::Dialog;

pub mod dropdown;
#[doc(inline)]
pub use dropdown::Dropdown;

pub mod form;
#[doc(inline)]
pub use form::Form;

mod html;
pub use html::Html;

pub mod image;
#[doc(inline)]
pub use image::Image;

mod intro;
pub use intro::{Intro, IntroOpening};

mod messages;
pub use messages::Messages;

pub mod nav;
#[doc(inline)]
pub use nav::Nav;

pub mod navbar;
#[doc(inline)]
pub use navbar::Navbar;

mod pager;
pub use pager::{Pager, PagerAlign, PagerVisibility};

mod poweredby;
pub use poweredby::PoweredBy;

pub mod table;
#[doc(inline)]
pub use table::Table;
