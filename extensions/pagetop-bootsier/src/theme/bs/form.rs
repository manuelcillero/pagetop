//! Definiciones para crear formularios ([`Form`]).

pub use pagetop::base::component::form::{Autocomplete, AutofillField, CheckboxKind, Method};

pub use pagetop::base::component::form::Form;

pub use pagetop::base::component::form::Fieldset;

pub use pagetop::base::component::form::Checkbox;

pub use pagetop::base::component::form::check;

pub use pagetop::base::component::form::radio;

pub mod select;
#[doc(inline)]
pub use select::SelectBsExt;

pub use pagetop::base::component::form::SelectLanguage;

pub use pagetop::base::component::form::SelectTheme;

pub use pagetop::base::component::form::SelectTimezone;

pub mod input;
#[doc(inline)]
pub use input::InputBsExt;

pub mod textarea;
pub use textarea::Textarea;
#[doc(inline)]
pub use textarea::TextareaBsExt;

pub use pagetop::base::component::form::date;

pub use pagetop::base::component::form::Number;

pub use pagetop::base::component::form::Range;

pub use pagetop::base::component::form::Hidden;
