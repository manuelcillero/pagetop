//! Re-exporta recursos comunes.

// Macros.
pub use crate::{
    args,
    concat_string,
    module_name,
    theme_static_files,
};

pub use crate::config::SETTINGS;
pub use crate::trace;
pub use crate::localize;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub use crate::{db, db::*, boxed_migration};

pub use crate::html::*;
pub use crate::theme::*;
pub use crate::module::*;
pub use crate::response::page::*;

pub use crate::app;
pub use crate::app::application::{Application, essence};

pub use crate::base::component::*;

pub use crate::util;
