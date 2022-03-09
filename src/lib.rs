// Global.

pub use doc_comment::doc_comment;
pub use once_cell::sync::Lazy;

// -----------------------------------------------------------------------------
// APIs públicas.
// -----------------------------------------------------------------------------

pub mod config;     // Gestión de la configuración.
pub mod trace;      // Registro de trazas y eventos de la aplicación.
pub mod locale;     // Localización.
pub mod database;   // Acceso a la base de datos.
pub mod core;       // Servidor web y sistemas para Temas, Módulos y Respuestas.
pub mod base;       // Temas, Módulos y Componentes base.
pub mod util;       // Macros y funciones útiles.

pub mod prelude;    // Re-exporta recursos comunes.

pub use crate::core::server::Application;
