//! Definiciones para crear formularios ([`Form`]).

pub use pagetop::base::component::form::{Autocomplete, AutofillField, CheckboxKind, Method};

pub use pagetop::base::component::form::Form;

pub use pagetop::base::component::form::Fieldset;

pub use pagetop::base::component::form::Checkbox;

pub use pagetop::base::component::form::check;

pub use pagetop::base::component::form::radio;

pub mod select;
#[doc(inline)]
pub use select::SelectBootsier;

pub mod input;
#[doc(inline)]
pub use input::InputBootsier;

pub mod textarea;
pub use textarea::Textarea;
#[doc(inline)]
pub use textarea::TextareaBootsier;

pub use pagetop::base::component::form::Range;

pub use pagetop::base::component::form::Hidden;
