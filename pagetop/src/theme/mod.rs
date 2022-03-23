mod definition;
pub use definition::ThemeTrait;

pub(crate) mod all;
pub use all::{
    register_theme,
    theme_by_name,
};
