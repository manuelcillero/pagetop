//! Re-exporta los tipos y funciones más habituales para la creación de soluciones web con
//! **PageTop**.

// Re-exports.
pub use crate::{concat_string, fn_builder, paste, Handle, HashMapResources, LazyStatic, ResultExt};

// Macros declarativas globales.
pub use crate::{args, define_config, define_handle, define_locale, serve_static_files};

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

pub use crate::{action, action_before_render_component};

pub use crate::base::component::*;

pub use crate::server;
pub use crate::server::HttpMessage;

pub use crate::response::fatal_error::*;
pub use crate::response::{page::*, ResponseError};

pub use crate::app::Application;
