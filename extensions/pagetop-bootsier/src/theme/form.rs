//! Definiciones para crear formularios ([`Form`]).

mod props;
pub use props::Method;
pub use props::{Autocomplete, AutofillField};
pub use props::CheckboxKind;

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
