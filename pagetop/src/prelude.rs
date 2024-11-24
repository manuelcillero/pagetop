//! The `PageTop` Prelude.

// RE-EXPORTED.

pub use crate::{concat_string, fn_builder, html, main, paste, test};

pub use crate::{AutoDefault, StaticResources, TypeId, Weight};

// MACROS.

// crate::util
pub use crate::{include_config, kv};
// crate::locale
pub use crate::include_locales;
// crate::service
pub use crate::{include_files, include_files_service};
// crate::core::action
pub use crate::actions;

// API.

pub use crate::util;

pub use crate::trace;

pub use crate::html::*;

pub use crate::locale::*;

pub use crate::service;
pub use crate::service::{HttpMessage, HttpRequest};

pub use crate::core::{AnyBase, AnyTo};

pub use crate::core::action::*;
pub use crate::core::package::*;
pub use crate::core::theme::*;

pub use crate::response::{json::*, page::*, redirect::*, ResponseError};

pub use crate::global;

pub use crate::app::Application;
