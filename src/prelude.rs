//! The PageTop Prelude.

// RE-EXPORTED MACROS AND DERIVES.
pub use crate::{concat_string, fn_builder, main, paste, test};
pub use crate::{AutoDefault, ComponentClasses};

// GLOBAL.
pub use crate::{HashMapResources, LazyStatic, TypeId, Weight};

// MACROS.

// crate::config
pub use crate::default_settings;
// crate::html
pub use crate::html;
// crate::locale
pub use crate::static_locales;
// crate::service
pub use crate::{service_for_static_files, static_files};
// crate::core::action
pub use crate::actions;
// crate::util
pub use crate::kv;

// API.

pub use crate::util;

pub use crate::config;

pub use crate::trace;

pub use crate::html::*;

pub use crate::locale::*;

pub use crate::datetime::*;

#[cfg(feature = "database")]
pub use crate::{db, db::*, migrations, new_migration};

pub use crate::service;
pub use crate::service::HttpMessage;

pub use crate::core::AnyBase;

pub use crate::core::action::*;
pub use crate::core::component::*;
pub use crate::core::package::*;
pub use crate::core::theme::*;

pub use crate::response::{json::*, page::*, redirect::*, ResponseError};

pub use crate::base::action;
pub use crate::base::component::*;
pub use crate::base::theme;

pub use crate::app::Application;
