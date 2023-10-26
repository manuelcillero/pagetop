//! The PageTop Prelude.

// Re-exported macros.
pub use crate::{concat_string, fn_builder, main, paste, test};

// Global.
pub use crate::{Handle, HashMapResources, LazyStatic, Weight};

// Functions and macro helpers.
pub use crate::util;
pub use crate::{kv, new_handle};

// MACROS.

// crate::config
pub use crate::default_settings;
// crate::locale
pub use crate::static_locales;
// crate::service
pub use crate::{static_files, static_files_service};
// crate::core::actions
pub use crate::actions;
// crate::base::action::component
pub use crate::actions_for_component;

// API.

pub use crate::config;

pub use crate::trace;

pub use crate::html::*;

pub use crate::locale::*;

pub use crate::datetime::*;

#[cfg(feature = "database")]
pub use crate::{db, db::*, migrations, pub_migration};

pub use crate::service;
pub use crate::service::HttpMessage;

pub use crate::core::action::*;
pub use crate::core::component::*;
pub use crate::core::module::*;
pub use crate::core::theme::*;

pub use crate::response::fatal_error::*;
pub use crate::response::{page::*, redirect::*, ResponseError};

pub use crate::base::action;
pub use crate::base::component::*;
pub use crate::base::theme;

pub use crate::app::Application;
