mod definition;
pub use definition::{ThemeStaticRef, ThemeTrait};

mod regions;
pub use regions::add_component_to;
pub(crate) use regions::ComponentsRegions;

pub(crate) mod all;

pub(crate) mod monster;
