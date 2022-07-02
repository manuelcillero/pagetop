// External re-exports.

pub use once_cell::sync::Lazy;
pub use concat_string::concat_string;
pub use doc_comment::doc_comment;

// Local.

#[allow(unused_imports)]
pub(crate) use futures::executor::block_on as run_now;

// Public APIs.

pub mod config;     // Gestión de la configuración.
pub mod trace;      // Registro de trazas y eventos de la aplicación.
pub mod locale;     // Localización.
pub mod html;       // HTML en código.

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub mod db;         // Acceso a base de datos.

pub mod app;        // Prepare and run the application.

pub mod core;       // Main APIs for components, hooks, modules and themes.

pub mod response;   // Tipos de respuestas web.
pub mod base;       // Base de componentes, módulos y temas.
pub mod util;       // Macros y funciones útiles.

// Internal re-exports.

pub mod prelude;
