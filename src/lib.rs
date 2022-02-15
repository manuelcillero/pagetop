// Global.

pub use doc_comment::doc_comment;
pub use once_cell::sync::Lazy;

// -----------------------------------------------------------------------------
// APIs públicas.
// -----------------------------------------------------------------------------

pub mod macros;     // Macros útiles.
pub mod config;     // Gestión de la configuración.
pub mod trace;      // Traza de ejecución.
pub mod locale;     // Localización.
pub mod core;       // Servidor web y sistemas para Temas, Módulos y Respuestas.
pub mod base;       // Temas, Módulos y Componentes base.

pub mod prelude;    // Re-exporta recursos comunes.
