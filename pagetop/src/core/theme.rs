mod definition;
pub use definition::{ThemeRef, ThemeTrait};

mod regions;
pub(crate) use regions::ComponentsRegions;
pub use regions::{add_component_in, Region};

mod default;
pub use default::DefaultTheme;

pub(crate) mod all;
