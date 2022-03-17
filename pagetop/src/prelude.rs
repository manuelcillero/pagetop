//! Re-exporta recursos comunes.

pub use crate::args;
pub use crate::config::SETTINGS;
pub use crate::trace;
pub use crate::localize;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub use crate::{db, boxed_migration};

pub use crate::core::html::*;
pub use crate::core::theme::*;
pub use crate::core::module::*;
pub use crate::core::response::page::*;
pub use crate::core::server;
pub use crate::core::server::app::{Application, essence};

pub use crate::base::component::*;

pub use crate::util;
