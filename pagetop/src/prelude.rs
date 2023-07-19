//! The PageTop Prelude.

// Re-exported macros.
pub use crate::{concat_string, fn_builder, main, paste, test};

// Global.
pub use crate::{Handle, HashMapResources, LazyStatic, ResultExt, Weight};

// Funciones y macros útiles.
pub use crate::util;
pub use crate::{action, actions_for_component};
pub use crate::{create_handle, default_settings, kv};
pub use crate::{serve_static_files, static_files, static_locales};

// API.

pub use crate::config;

pub use crate::trace;

pub use crate::locale::*;

pub use crate::html::*;

#[cfg(feature = "database")]
pub use crate::{db, db::*, migration_item, pub_migration};

pub use crate::core::action::*;
pub use crate::core::component::html::*;
pub use crate::core::component::l10n::*;
pub use crate::core::component::*;
pub use crate::core::module::*;
pub use crate::core::theme::*;

pub use crate::service;
pub use crate::service::HttpMessage;

pub use crate::response::fatal_error::*;
pub use crate::response::{page::*, ResponseError};

pub use crate::app::Application;
