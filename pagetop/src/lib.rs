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
//!  * Localización ([`locale`]).
//!
//!  * HTML en código ([`html`]).
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
//! # 🏃‍♀️ Inicio rápido
//!
//! Puedes encontrar este código en el repositorio de ejemplos
//! [básicos](https://github.com/manuelcillero/pagetop/tree/main/examples/basics) de PageTop:
//! ```rust
//! use pagetop::prelude::*;
//!
//! define_handle!(APP_HELLO_WORLD);
//!
//! struct HelloWorld;
//!
//! impl ModuleTrait for HelloWorld {
//!     fn handle(&self) -> Handle {
//!         APP_HELLO_WORLD
//!     }
//!
//!     fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
//!         cfg.service(hello_world);
//!     }
//! }
//!
//! #[service::get("/")]
//! async fn hello_world(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
//!     Page::new(request)
//!         .with_in("content", Html::with(html! { h1 { "Hello World!" } }))
//!         .render()
//! }
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     Application::prepare(&HelloWorld).unwrap().run()?.await
//! }
//! ```
//! Este programa crea un módulo llamado `HelloWorld` con un servicio que devuelve una página web
//! saludando al mundo cada vez que se accede desde el navegador a `http://localhost:8088` (según
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

// *************************************************************************************************
// GLOBAL.
// *************************************************************************************************

pub use concat_string::concat_string;
pub use doc_comment::doc_comment;
pub use once_cell::sync::Lazy as LazyStatic;
pub use paste::paste;
pub use static_files::Resource as StaticResource;
pub use tracing_unwrap::ResultExt;

#[allow(unused_imports)]
pub(crate) use futures::executor::block_on as run_now;

pub use pagetop_macros::fn_builder;

pub type HashMapResources = std::collections::HashMap<&'static str, StaticResource>;

pub type Handle = u64;

define_locale!(LOCALE_PAGETOP, "static/locales");

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

// APIs esenciales para crear acciones, componentes, módulos y temas.
pub mod core;

// Acciones, componentes, módulos y temas integrados en PageTop.
pub mod base;

// API para operar con los servicios web.
pub mod service;

// Tipos de respuestas a peticiones web.
pub mod response;

// Funciones útiles y macros declarativas.
pub mod util;

// Prepara y ejecuta la aplicación.
pub mod app;

// *************************************************************************************************
// RE-EXPORTA API ÚNICA.
// *************************************************************************************************

pub mod prelude;
