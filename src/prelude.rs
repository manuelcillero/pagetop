//! *Prelude* de PageTop.

// RE-EXPORTED.

pub use crate::{builder_fn, html, main, test};

pub use crate::{AutoDefault, StaticResources, UniqueId, Weight};

// MACROS.

// crate::util
pub use crate::{hm, join, join_opt, join_pair, join_strict};
// crate::config
pub use crate::include_config;
// crate::locale
pub use crate::include_locales;
// crate::service
#[allow(deprecated)]
pub use crate::{include_files, include_files_service, static_files_service};
// crate::core::action
pub use crate::actions_boxed;

// API.

pub use crate::util;

pub use crate::global;

pub use crate::trace;

// No se usa `pub use crate::html::*;` para evitar duplicar alias marcados como obsoletos
// (*deprecated*) porque han sido trasladados a `crate::core::component`. Cuando se retiren estos
// alias obsoletos se volverá a declarar como `pub use crate::html::*;`.
pub use crate::html::{
    display, html_private, Asset, Assets, AttrClasses, AttrId, AttrL10n, AttrName, AttrValue,
    ClassesOp, Escaper, Favicon, JavaScript, Markup, PreEscaped, PrepareMarkup, StyleSheet,
    TargetMedia, DOCTYPE,
};

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
