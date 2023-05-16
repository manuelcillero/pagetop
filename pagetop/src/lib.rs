//! <div align="center">
//!
//!   <img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/banner/pagetop.png" />
//!
//!   <h1>PageTop</h1>
//!
//!   [![crate](https://img.shields.io/crates/v/pagetop.svg)](https://crates.io/crates/pagetop)
//!   [![docs](https://docs.rs/pagetop/badge.svg)](https://docs.rs/pagetop)
//!
//! </div>
//!
//! **PageTop** es un entorno de desarrollo basado en Rust que reúne algunos de los crates más
//! estables y populares para crear soluciones web modulares, extensibles y configurables.
//!
//! PageTop define un interfaz único que ofrece:
//!
//!  * Lectura y uso de ajustes de configuración ([`config`]).
//!
//!  * Registro de trazas y eventos de la aplicación ([`trace`]).
//!
//!  * Localización ([`locale`]).
//!
//!  * HTML en código ([`html`]).
//!
//!  * Acceso unificado y normalizado a base de datos ([`db`]).
//!
//!  * Tipos y funciones esenciales para crear módulos, temas, componentes y acciones ([`core`]).
//!
//!  * Tipos de respuestas a peticiones web ([`response`])
//!
//!  * Funciones útiles ([`util`]).
//!
//! # 🚧 Advertencia
//!
//! **PageTop** sólo libera actualmente versiones de desarrollo. La API no es estable y los cambios
//! son constantes. No puede considerarse preparado hasta que se libere la versión **0.1.0**.

// *************************************************************************************************
// GLOBAL.
// *************************************************************************************************

pub use concat_string::concat_string;
pub use doc_comment::doc_comment;
pub use once_cell::sync::Lazy as LazyStatic;
pub use paste::paste;
pub use static_files::Resource as StaticResource;
pub use tracing_unwrap::ResultExt;

pub use pagetop_macros::fn_builder;

pub type HashMapResources = std::collections::HashMap<&'static str, StaticResource>;

pub type Handle = u64;

// *************************************************************************************************
// LOCAL.
// *************************************************************************************************

#[allow(unused_imports)]
pub(crate) use futures::executor::block_on as run_now;

// *************************************************************************************************
// APIs PÚBLICAS.
// *************************************************************************************************

// Gestión de la configuración.
pub mod config;
// Registro de trazas y eventos de la aplicación.
pub mod trace;
// Localización.
pub mod locale;
// HTML en código.
pub mod html;

// Acceso a base de datos.
#[cfg(feature = "database")]
pub mod db;

// APIs esenciales para crear módulos, temas, componentes y acciones.
pub mod core;

// API del servidor web.
pub mod server;

// Tipos de respuestas a peticiones web.
pub mod response;

// Funciones útiles.
pub mod util;

// Prepara y ejecuta la aplicación.
pub mod app;

// *************************************************************************************************
// MACROS DECLARATIVAS.
// *************************************************************************************************

#[macro_export]
/// Macro para construir grupos de pares clave-valor.
///
/// ```rust#ignore
/// let args = args![
///     "userName" => "Roberto",
///     "photoCount" => 3,
///     "userGender" => "male"
/// ];
/// ```
macro_rules! args {
    ( $($key:expr => $value:expr),* ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert(String::from($key), $value.into());
        )*
        a
    }};
}

#[macro_export]
macro_rules! define_handle {
    ( $HANDLE:ident ) => {
        pub const $HANDLE: $crate::Handle =
            $crate::util::handle(module_path!(), file!(), line!(), column!());
    };
}

#[macro_export]
macro_rules! serve_static_files {
    ( $cfg:ident, $dir:expr, $embed:ident ) => {{
        let static_files = &$crate::config::SETTINGS.dev.static_files;
        if static_files.is_empty() {
            $cfg.service($crate::server::ResourceFiles::new($dir, $embed()));
        } else {
            $cfg.service(
                $crate::server::ActixFiles::new($dir, $crate::concat_string!(static_files, $dir))
                    .show_files_listing(),
            );
        }
    }};
}

// *************************************************************************************************
// RE-EXPORTA API ÚNICA.
// *************************************************************************************************

pub mod prelude;
