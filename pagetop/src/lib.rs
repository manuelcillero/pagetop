// Global.

pub use doc_comment::doc_comment;
pub use once_cell::sync::Lazy;
pub use futures::executor::block_on as run_now;

// -----------------------------------------------------------------------------
// APIs públicas.
// -----------------------------------------------------------------------------

pub mod config;     // Gestión de la configuración.
pub mod trace;      // Registro de trazas y eventos de la aplicación.
pub mod locale;     // Localización.
pub mod db;         // Acceso a la base de datos.
pub mod core;       // Servidor web y APIs para Temas, Módulos y Respuestas web.
pub mod base;       // Temas, Módulos y Componentes base.
pub mod util;       // Macros y funciones útiles.

pub mod prelude;    // Re-exporta recursos comunes.

pub use crate::core::server::Application;
