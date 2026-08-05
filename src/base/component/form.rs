//! Definiciones para crear formularios ([`Form`]).

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

mod number;
pub use number::Number;

mod range;
pub use range::Range;

mod textarea;
pub use textarea::Textarea;

mod hidden;
pub use hidden::Hidden;
