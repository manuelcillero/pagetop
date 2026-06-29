//! Definiciones para crear formularios ([`Form`]).

pub use pagetop::base::component::form::{Autocomplete, AutofillField, CheckboxKind, Method};

pub use pagetop::base::component::form::Form;

pub use pagetop::base::component::form::Fieldset;

pub use pagetop::base::component::form::Checkbox;

pub use pagetop::base::component::form::check;

pub use pagetop::base::component::form::radio;

pub mod select;

pub mod input;

pub mod textarea;
pub use textarea::Textarea;

pub use pagetop::base::component::form::Range;

pub use pagetop::base::component::form::Hidden;
