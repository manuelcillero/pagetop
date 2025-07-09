//! *Prelude* de `PageTop`.

// RE-EXPORTED.

pub use crate::{html, main, test};

pub use crate::AutoDefault;

// MACROS.

// crate::util
pub use crate::hm;
// crate::config
pub use crate::include_config;
// crate::locale
pub use crate::include_locales;

// API.

pub use crate::util;

pub use crate::global;

pub use crate::trace;

pub use crate::html::*;

pub use crate::locale::*;

pub use crate::datetime::*;

pub use crate::service;

pub use crate::app::Application;
