//! Re-exporta los tipos y funciones más habituales para la creación de soluciones web con
//! **PageTop**.

// Re-exports.
pub use crate::{
    concat_string, fn_builder, paste, Handle, HashMapResources, LazyStatic, ResultExt,
};

// Macros declarativas globales.
pub use crate::{args, serve_static_files, use_config, use_handle, use_locale, use_static};

// Traducciones globales.
pub use crate::LOCALE_PAGETOP;

// Funciones útiles.
pub use crate::util;

// *************************************************************************************************

pub use crate::config;

pub use crate::trace;

pub use crate::locale::*;

pub use crate::html::*;

#[cfg(feature = "database")]
pub use crate::{db, db::*, migration_item, pub_migration};

pub use crate::core::{action::*, component::*, module::*, theme::*};

pub use crate::{action, action_after_prepare_component, action_before_prepare_component};

pub use crate::base::actions;
pub use crate::base::components::*;
pub use crate::base::themes;

pub use crate::service;
pub use crate::service::HttpMessage;

pub use crate::response::fatal_error::*;
pub use crate::response::{page::*, ResponseError};

pub use crate::app::Application;
