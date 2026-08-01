//! Componentes nativos proporcionados por PageTop.

pub mod layout;

mod badge;
pub use badge::Badge;

mod block;
pub use block::Block;

mod button;
pub use button::{Button, ButtonAction};

pub mod container;
#[doc(inline)]
pub use container::Container;

pub mod form;
#[doc(inline)]
pub use form::Form;

mod html;
pub use html::Html;

mod intro;
pub use intro::{Intro, IntroOpening};

mod poweredby;
pub use poweredby::PoweredBy;
