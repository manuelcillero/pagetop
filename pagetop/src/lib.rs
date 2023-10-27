//! <div align="center">
//!
//!   <img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/banner-pagetop.png" />
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
//! PageTop define un interfaz único para aplicaciones SSR (*Server-Side Rendering*) que ofrece:
//!
//!  * Lectura y uso de ajustes de configuración ([`config`]).
//!
//!  * Registro de trazas y eventos de la aplicación ([`trace`]).
//!
//!  * HTML en código ([`html`]).
//!
//!  * Localización ([`locale`]).
//!
//!  * [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) date and time handling ([`datetime`]).
//!
//!  * Acceso unificado y normalizado a base de datos ([`db`]).
//!
//!  * Tipos y funciones esenciales para crear acciones, componentes, módulos y temas ([`core`]).
//!
//!  * Tipos de respuestas a peticiones web ([`response`])
//!
//!  * Funciones útiles ([`util`]).
//!
//! # 🚧 Advertencia
//!
//! **PageTop** sólo libera actualmente versiones de desarrollo. La API no es estable y los cambios
//! son constantes. No puede considerarse preparado hasta que se libere la versión **0.1.0**.
//!
//! # ⚡️ Inicio rápido
//!
//! Puedes encontrar este código en el repositorio de ejemplos
//! [básicos](https://github.com/manuelcillero/pagetop/tree/main/examples/basics) de PageTop:
//! ```rust
//! use pagetop::prelude::*;
//!
//! new_handle!(APP_HELLO_WORLD);
//!
//! struct HelloWorld;
//!
//! impl ModuleTrait for HelloWorld {
//!     fn handle(&self) -> Handle {
//!         APP_HELLO_WORLD
//!     }
//!
//!     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
//!         scfg.route("/", service::web::get().to(hello_world));
//!     }
//! }
//!
//! async fn hello_world(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
//!     Page::new(request)
//!         .with_in("content", Html::with(html! { h1 { "Hello World!" } }))
//!         .render()
//! }
//!
//! #[pagetop::main]
//! async fn main() -> std::io::Result<()> {
//!     Application::prepare(&HelloWorld).unwrap().run()?.await
//! }
//! ```
//! Este programa crea un módulo llamado `HelloWorld` con un servicio que devuelve una página web
//! saludando al mundo cada vez que se accede desde el navegador a `http://localhost:8088` (para
//! los [ajustes de configuración](`config::Server`) predeterminados).
//!
//! # 🧱 Extendiendo PageTop
//!
//! La API de PageTop no sólo sirve para crear aplicaciones, también permite extender sus
//! funcionalidades con el desarrollo de acciones, componentes, módulos o nuevos temas:
//!
//! * Las **acciones** permiten alterar el comportamiento del propio funcionamiento interno de
//!   PageTop. Las acciones se ofrecen al desarrollador para interactuar con la ejecución de
//!   procesos que pueden ser modificados.
//! * Los **componentes** incluyen código HTML, CSS y/o Javascript en unidades que tienen una
//!   funcionalidad bien definida y configurable durante la creación de páginas web.
//! * Los **módulos** añaden funcionalidades o modifican las ya existentes usando las APIs globales
//!   o las de otros módulos de PageTop o de terceros.
//! * Los **temas** son módulos que permiten cambiar la disposición y el aspecto de las páginas y
//!   componentes sin necesidad de alterar su funcionamiento. Estructuran las páginas en regiones
//!   donde disponer los diferentes componentes.
//!
//! # 🧩 Dependencias
//!
//! Las aplicaciones usarán `cargo` para resolver las dependencias entre PageTop y las extensiones
//! de terceros que implementen acciones, componentes, módulos y/o temas; de la misma manera que se
//! hace en cualquier otro proyecto.
//!
//! Pero también deberán declararse explícitamente estas dependencias en cada módulo para ayudar a
//! PageTop a estructurar e inicializar modularmente la aplicación.

#![cfg_attr(docsrs, feature(doc_cfg))]

// *************************************************************************************************
// RE-EXPORTED MACROS.
// *************************************************************************************************

pub use concat_string::concat_string;

/// Enables flexible identifier concatenation in macros, allowing new items with pasted identifiers.
pub use paste::paste;

pub use pagetop_macros::{fn_builder, main, test};

// *************************************************************************************************
// GLOBAL.
// *************************************************************************************************

pub use once_cell::sync::Lazy as LazyStatic;
pub use static_files::Resource as StaticResource;

pub type Handle = u64;
pub type Weight = i8;
pub type HashMapResources = std::collections::HashMap<&'static str, StaticResource>;

new_static_locales!(LOCALES_PAGETOP);

// *************************************************************************************************
// PUBLIC API.
// *************************************************************************************************

// Functions and macro helpers.
pub mod util;

// Gestión de la configuración.
pub mod config;
// Registro de trazas y eventos de la aplicación.
pub mod trace;
// HTML en código.
pub mod html;
// Localización.
pub mod locale;
// Date and time for PageTop.
pub mod datetime;

// Acceso a base de datos.
#[cfg_attr(docsrs, doc(cfg(feature = "database")))]
#[cfg(feature = "database")]
pub mod db;

// API para operar con los servicios web.
pub mod service;

// APIs esenciales para crear acciones, componentes, módulos y temas.
pub mod core;

// Tipos de respuestas a peticiones web.
pub mod response;

// Base de acciones, componentes, módulos y temas.
pub mod base;

// Prepara y ejecuta la aplicación.
pub mod app;

// *************************************************************************************************
// The PageTop Prelude.
// *************************************************************************************************

pub mod prelude;
