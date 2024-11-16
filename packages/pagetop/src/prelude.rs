//! The `PageTop` Prelude.

// RE-EXPORTED.

pub use crate::{join, main, paste, test};

pub use crate::{AutoDefault, StaticResources, TypeId, Weight};

// MACROS.

// crate::util
pub use crate::{kv, static_config};
// crate::locale
pub use crate::static_locales;
// crate::service
pub use crate::{static_files, static_files_service};
// crate::core::action
pub use crate::actions;

// API.

pub use crate::util;

pub use crate::trace;

pub use crate::locale::*;

pub use crate::service;
pub use crate::service::{HttpMessage, HttpRequest};

pub use crate::core::{AnyBase, AnyTo};

pub use crate::core::action::*;
pub use crate::core::package::*;

pub use crate::response::{json::*, redirect::*, ResponseError};

pub use crate::global;

pub use crate::app::Application;
