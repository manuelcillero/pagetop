//! The `PageTop` Prelude.

// RE-EXPORTED MACROS AND DERIVES.
pub use crate::{concat_string, fn_builder, main, paste, test};
pub use crate::{AutoDefault, ComponentClasses};

// GLOBAL.
pub use crate::{global, HashMapResources, TypeId, Weight};

// MACROS.

// crate::global
pub use crate::kv;
// crate::config
pub use crate::config_defaults;
// crate::html
pub use crate::html;
// crate::locale
pub use crate::static_locales;
// crate::service
pub use crate::{static_files, static_files_service};
// crate::core::action
pub use crate::actions;

// API.

pub use crate::trace;

pub use crate::html::*;

pub use crate::locale::*;

pub use crate::datetime::*;

pub use crate::service;
pub use crate::service::{HttpMessage, HttpRequest};

pub use crate::core::{AnyBase, AnyTo};

pub use crate::core::action::*;
pub use crate::core::component::*;
pub use crate::core::package::*;
pub use crate::core::theme::*;

pub use crate::response::{json::*, page::*, redirect::*, ResponseError};

pub use crate::base::action;
pub use crate::base::component::*;
pub use crate::base::theme;

pub use crate::app::Application;
