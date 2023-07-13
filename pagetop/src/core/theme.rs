mod definition;
pub use definition::{ThemeStaticRef, ThemeTrait};

mod regions;
pub use regions::add_component_to;
pub(crate) use regions::ComponentsRegions;

mod basic;
pub(crate) use basic::Basic;

pub(crate) mod all;
