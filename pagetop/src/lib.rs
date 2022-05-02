// External.

pub use concat_string::concat_string;
pub use doc_comment::doc_comment;
pub use once_cell::sync::Lazy;

// Local.

pub(crate) use futures::executor::block_on as run_now;

// Public APIs.

pub mod config;     // Gestión de la configuración.
pub mod trace;      // Registro de trazas y eventos de la aplicación.
pub mod locale;     // Localización.
pub mod html;       // HTML en código.

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub mod db;         // Acceso a base de datos.

pub mod core;       // APIs para módulos, respuestas web y temas.
pub mod app;        // Aplicación y servidor web.
pub mod base;       // Base de componentes, módulos y temas.
pub mod util;       // Macros y funciones útiles.

// Re-exports.

pub mod prelude;
