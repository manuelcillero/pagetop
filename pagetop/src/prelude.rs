// Global macros and helpers.
pub use crate::util;
pub use crate::util::{Handler, HashMapResources};
pub use crate::{
    args, concat_string, configure_service_for_static_files, pub_const_handler, LazyStatic,
};

pub use crate::config::SETTINGS;

pub use crate::trace;

pub use crate::localize;

pub use crate::html::*;

#[cfg(feature = "database")]
pub use crate::{db, db::*, migration_item, pub_migration};

pub use crate::app;
pub use crate::app::application::Application;
pub use crate::app::fatal_error::FatalError;
pub use crate::app::HttpMessage;

pub use crate::core::{component::*, hook::*, module::*, theme::*};

pub use crate::{hook_action, hook_before_render_component};

pub use crate::response::page::*;
pub use crate::response::ResponseError;

pub use crate::base::component::*;
