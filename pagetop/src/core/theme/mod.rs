mod definition;
pub use definition::{
    BaseTheme,
    ThemeTrait,
};

pub(crate) mod all;
pub use all::{
    register_theme,
    register_themes,
    theme_by_single_name,
};
