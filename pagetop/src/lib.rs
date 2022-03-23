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

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub mod db;         // Acceso a la base de datos.

pub mod html;       // Publicación de HTML desde el código.
pub mod module;     // API para crear módulos con nuevas funcionalidades.
pub mod theme;      // API para crear temas y temas predeterminados.
pub mod response;   // Tipos de respuestas web.
pub mod app;        // Aplicación y servidor web.

pub mod base;       // Componentes, Módulos y Temas base.
pub mod util;       // Macros y funciones útiles.

pub mod prelude;    // Re-exporta recursos comunes.
