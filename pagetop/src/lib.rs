//! <div align="center">
//!
//!   <img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/pagetop/static/pagetop-banner.png" />
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
//! PageTop añade una capa de abstracción para definir una interfaz única que ofrezca de partida:
//!
//!  * Gestión de la configuración ([`config`]).
//!
//!  * Registro de trazas y eventos de la aplicación ([`trace`]).
//!
//!  * Localización ([`locale`]).
//!
//!  * HTML en código ([`html`]).
//!
//!  * Acceso a base de datos ([`db`]).
//!
//!  * APIs esenciales para crear componentes, acciones, módulos y temas ([`core`]).
//!
//!  * Tipos de respuestas a peticiones web ([`response`])
//!
//!  * Base de componentes, módulos y temas ([`base`]).
//!
//!  * Utilidades de carácter global ([`util`]).
//!
//! # 🚧 Advertencia
//!
//! **PageTop** sólo libera actualmente versiones de desarrollo. La API no es estable y los cambios
//! son constantes. No puede considerarse preparado hasta que se libere la versión **0.1.0**.

// GLOBAL.

pub use concat_string::concat_string;
pub use doc_comment::doc_comment;
pub use once_cell::sync::Lazy as LazyStatic;
pub use paste::paste;
pub use tracing_unwrap::ResultExt;

pub use pagetop_macros::fn_builder;

// LOCAL.

#[allow(unused_imports)]
pub(crate) use futures::executor::block_on as run_now;

// APIs PÚBLICAS.

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

// APIs esenciales para crear componentes, acciones, módulos y temas.
pub mod core;

// API del servidor web.
pub mod server;

// Tipos de respuestas a peticiones web.
pub mod response;

// Macros y funciones útiles.
pub mod util;

// Prepara y ejecuta la aplicación.
pub mod app;

// RE-EXPORTA API ÚNICA.

pub mod prelude;
