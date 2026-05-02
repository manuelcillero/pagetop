//! Definiciones para crear formularios ([`Form`]).
//!
//! # Ejemplo
//!
//! ```rust
//! use pagetop::prelude::*;
//! use pagetop_bootsier::prelude::*;
//!
//! let form_login = Form::new()
//!     .with_id("login")
//!     .with_action("/login")
//!     .with_child(
//!         form::input::Field::email()
//!             .with_name("email")
//!             .with_label(L10n::n("Email"))
//!             .with_required(true),
//!     )
//!     .with_child(
//!         form::input::Field::password()
//!             .with_name("password")
//!             .with_label(L10n::n("Password"))
//!             .with_required(true),
//!     )
//!     .with_child(
//!         form::Checkbox::check()
//!             .with_name("remember")
//!             .with_label(L10n::n("Remember me")),
//!     )
//!     .with_child(
//!         Button::submit(L10n::n("Sign in"))
//!             .with_color(ButtonColor::Background(Color::Primary)),
//!     );
//! ```

mod props;
pub use props::{Autocomplete, AutofillField, CheckboxKind, Method};

mod component;
pub use component::Form;

mod fieldset;
pub use fieldset::Fieldset;

mod checkbox;
pub use checkbox::Checkbox;

pub mod check;

pub mod radio;

pub mod select;

pub mod input;

mod textarea;
pub use textarea::Textarea;

mod range;
pub use range::Range;

mod hidden;
pub use hidden::Hidden;
