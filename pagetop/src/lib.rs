// EXTERNAL RE-EXPORTS.

pub use concat_string::concat_string;
pub use doc_comment::doc_comment;
pub use once_cell::sync::Lazy;

// LOCAL.

#[allow(unused_imports)]
pub(crate) use futures::executor::block_on as run_now;

// PUBLIC APIs.

// Gestión de la configuración.
pub mod config;
// Registro de trazas y eventos de la aplicación.
pub mod trace;
// Localización.
pub mod locale;
// HTML en código.
pub mod html;

// Acceso a base de datos.
#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub mod db;

// Prepare and run the application.
pub mod app;

// Main APIs for components, hooks, modules and themes.
pub mod core;

// Tipos de respuestas web.
pub mod response;
// Base de componentes, módulos y temas.
pub mod base;
// Macros y funciones útiles.
pub mod util;

// INTERNAL RE-EXPORTS.

pub mod prelude;
