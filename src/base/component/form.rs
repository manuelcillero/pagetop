//! Definiciones para crear formularios ([`Form`]).

mod props;
pub use props::{Autocomplete, AutofillField, CheckboxKind, Method};

mod component;
pub use component::Form;

mod field_help;
pub use field_help::FieldHelp;

mod fieldset;
pub use fieldset::Fieldset;

mod checkbox;
pub use checkbox::Checkbox;

pub mod check;

pub mod radio;

pub mod select;

mod select_language;
pub use select_language::SelectLanguage;

mod select_theme;
pub use select_theme::SelectTheme;

mod select_timezone;
pub use select_timezone::SelectTimezone;

pub mod input;

pub mod date;

mod number;
pub use number::Number;

mod range;
pub use range::Range;

mod textarea;
pub use textarea::Textarea;

mod hidden;
pub use hidden::Hidden;
