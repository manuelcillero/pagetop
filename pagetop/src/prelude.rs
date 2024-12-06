//! *Prelude* de `PageTop`.

// RE-EXPORTED.

pub use crate::{fn_builder, html, main, test};

pub use crate::{AutoDefault, ComponentClasses, StaticResources, UniqueId, Weight};

// MACROS.

// crate::util
pub use crate::kv;
// crate::config
pub use crate::include_config;
// crate::locale
pub use crate::include_locales;
// crate::service
pub use crate::{include_files, include_files_service};
// crate::core::action
pub use crate::actions;

// API.

pub use crate::util;

pub use crate::global;

pub use crate::trace;

pub use crate::html::*;

pub use crate::locale::*;

pub use crate::datetime::*;

pub use crate::service;
pub use crate::service::{HttpMessage, HttpRequest};

pub use crate::core::{AnyBase, AnyTo};

pub use crate::core::action::*;
pub use crate::core::component::*;
pub use crate::core::layout::*;
pub use crate::core::package::*;

pub use crate::response::{json::*, page::*, redirect::*, ResponseError};

pub use crate::base::action;
pub use crate::base::component::*;
pub use crate::base::layout;

pub use crate::app::Application;
