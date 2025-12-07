//! *Prelude* de PageTop.

// RE-EXPORTED.

pub use crate::PAGETOP_VERSION;

pub use crate::{builder_fn, html, main, test};

pub use crate::{AutoDefault, Getters, StaticResources, UniqueId, Weight};

// MACROS.

// crate::config
pub use crate::include_config;
// crate::locale
pub use crate::include_locales;
// crate::service
pub use crate::static_files_service;
// crate::core::action
pub use crate::actions_boxed;

// API.

pub use crate::util;

pub use crate::global;

pub use crate::trace;

pub use crate::html::*;

pub use crate::locale::*;

pub use crate::datetime::*;

pub use crate::service;
pub use crate::service::{HttpMessage, HttpRequest, HttpResponse};

pub use crate::core::{AnyCast, AnyInfo, TypeInfo};

pub use crate::core::action::*;
pub use crate::core::component::*;
pub use crate::core::extension::*;
pub use crate::core::theme::*;

pub use crate::response::{json::*, page::*, redirect::*, ResponseError};

pub use crate::base::action;
pub use crate::base::component::*;
pub use crate::base::theme;

pub use crate::app::Application;
