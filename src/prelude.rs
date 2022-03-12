//! Re-exporta recursos comunes.

pub use crate::{
    args,
    db_migrations,
};

pub use crate::config::SETTINGS;
pub use crate::trace;
pub use crate::localize;

pub use crate::db;

pub use crate::core::theme::*;
pub use crate::core::module::*;
pub use crate::core::response::page::*;
pub use crate::core::server;

pub use crate::base::component::*;

pub use crate::util;
