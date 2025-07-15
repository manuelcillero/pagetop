//! *Prelude* de `PageTop`.

// RE-EXPORTED.

pub use crate::{builder_fn, html, main, test};

pub use crate::{AutoDefault, StaticResources};

// MACROS.

// crate::util
pub use crate::hm;
// crate::config
pub use crate::include_config;
// crate::locale
pub use crate::include_locales;
// crate::service
pub use crate::{include_files, include_files_service};

// API.

pub use crate::util;

pub use crate::global;

pub use crate::trace;

pub use crate::html::*;

pub use crate::locale::*;

pub use crate::datetime::*;

pub use crate::service;

pub use crate::core::{AnyCast, AnyInfo, TypeInfo};

pub use crate::core::extension::*;

pub use crate::app::Application;
