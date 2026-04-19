//! Definiciones para crear formularios ([`Form`]).

mod props;
pub use props::{Autocomplete, AutofillField};
pub use props::{CheckboxKind, InputType, Method};

mod component;
pub use component::Form;

mod fieldset;
pub use fieldset::Fieldset;

mod input;
pub use input::Input;

mod checkbox;
pub use checkbox::Checkbox;
