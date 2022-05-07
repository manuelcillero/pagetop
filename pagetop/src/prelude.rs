//! Re-exporta recursos comunes.

// Macros.
pub use crate::{
    args,
    concat_string,
    theme_static_files,
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

pub use crate::{action_item, api::{
    action::*,
    component::*,
    module::*,
    theme::*,
}};

pub use crate::response::page::*;

pub use crate::app;
pub use crate::app::application::{Application, UsingBootstrap};

pub use crate::base::component::*;

pub use crate::util;
