//! The `PageTop` Prelude.

// RE-EXPORTED.
pub use crate::{join, main, paste, test, AutoDefault};

// GLOBAL.
pub use crate::{global, HashMapResources, TypeId, Weight};

// MACROS.

// crate::global
pub use crate::kv;
// crate::config
pub use crate::config_defaults;
// crate::locale
pub use crate::static_locales;
// crate::service
pub use crate::{static_files, static_files_service};
// crate::core::action
pub use crate::actions;

// API.

pub use crate::trace;

pub use crate::locale::*;

pub use crate::service;
pub use crate::service::{HttpMessage, HttpRequest};

pub use crate::core::{AnyBase, AnyTo};

pub use crate::core::action::*;
pub use crate::core::package::*;

pub use crate::response::{json::*, redirect::*, ResponseError};

pub use crate::app::Application;
