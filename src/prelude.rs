//! *Prelude* de `PageTop`.

// RE-EXPORTED.

pub use crate::{builder_fn, html, main, test};

pub use crate::{AutoDefault, StaticResources, Weight};

// MACROS.

// crate::util
pub use crate::{hm, join, join_opt, join_pair, join_strict};
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
pub use crate::core::theme::*;

pub use crate::base::theme;

pub use crate::app::Application;
