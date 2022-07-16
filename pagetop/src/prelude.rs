//! Re-exporta recursos comunes.

// Macros, globals and helpers.
pub use crate::{
    args,
    concat_string,
    theme_static_files,
    util,
};

pub use crate::config::SETTINGS;
pub use crate::trace;
pub use crate::localize;
pub use crate::html::*;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub use crate::{
    db,
    db::*,
    pub_migration,
    migration_item,
};

pub use crate::app;
pub use crate::app::AppTrait;
pub use crate::app::application::Application;

pub use crate::{hook_action, core::{
    component::*,
    hook::*,
    module::*,
    theme::*,
}};

pub use crate::response::page::*;

pub use crate::base::component::*;
