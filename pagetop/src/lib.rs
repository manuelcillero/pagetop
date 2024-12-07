//! <div align="center">
//!
//! <img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/tools/banner.png" />
//!
//! <h1>PageTop</h1>
//!
//! <p>Entorno de desarrollo para crear soluciones web modulares, extensibles y configurables.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
//! [![Doc API](https://img.shields.io/docsrs/pagetop?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop)
//! [![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)
//! [![Descargas](https://img.shields.io/crates/d/pagetop.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop)
//!
//! <br>
//! </div>
//!
//! `PageTop` reivindica la sencillez de la web clásica utilizando SSR (*renderizado en el
//! servidor*), HTML, CSS y JavaScript. Proporciona un conjunto completo de funcionalidades que
//! pueden extenderse y adaptarse a las necesidades de cada solución web implementando:
//!
//!   * **Acciones** (*actions*). Las funcionalidades que incorporen acciones en su lógica de
//!     programa estarán proporcionando a los desarrolladores herramientas para alterar su
//!     comportamiento interno interceptando su flujo de ejecución.
//!   * **Componentes** (*components*). Para encapsular HTML, CSS y JavaScript en unidades
//!     funcionales, configurables y bien definidas.
//!   * **Diseños** (*layouts*). Permiten a los desarrolladores modificar la apariencia de páginas y
//!     componentes sin afectar a su funcionalidad.
//!   * **Paquetes** (*packages*). Extienden o personalizan funcionalidades existentes interactuando
//!     con las APIs de `PageTop` o de paquetes de terceros.
//!
//! # ⚡️ Inicio rápido
//!
//! La aplicación más sencilla de `PageTop` se ve así:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! #[pagetop::main]
//! async fn main() -> std::io::Result<()> {
//!     Application::new().run()?.await
//! }
//! ```
//!
//! Proporciona una página de bienvenida en `http://localhost:8088` según la configuración
//! predefinida. Para personalizar el servicio puedes crear un paquete de `PageTop`:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! struct HelloWorld;
//!
//! impl PackageTrait for HelloWorld {
//!     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
//!         scfg.route("/", service::web::get().to(hello_world));
//!     }
//! }
//!
//! async fn hello_world(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
//!     Page::new(request)
//!         .with_component(Html::with(html! { h1 { "Hello world!" } }))
//!         .render()
//! }
//!
//! #[pagetop::main]
//! async fn main() -> std::io::Result<()> {
//!     Application::prepare(&HelloWorld).run()?.await
//! }
//! ```
//!
//! Este programa prepara un paquete personalizado llamado `HelloWorld` que sirve una página web en
//! la ruta raíz (`/`) mostrando el mensaje "Hello world!" en un elemento HTML `<h1>`.
//!
//! # 🧩 Gestión de Dependencias
//!
//! Los proyectos que utilizan `PageTop` gestionan las dependencias con `cargo`, como cualquier otro
//! proyecto en Rust.
//!
//! Sin embargo, es fundamental que cada paquete declare explícitamente sus
//! [dependencias](core::package::PackageTrait#method.dependencies), si las tiene, para que
//! `PageTop` pueda estructurar e inicializar la aplicación de forma modular.

#![cfg_attr(docsrs, feature(doc_cfg))]

// RE-EXPORTED *************************************************************************************

pub use pagetop_macros::{fn_builder, html, main, test, AutoDefault, ComponentClasses};

pub type StaticResources = std::collections::HashMap<&'static str, static_files::Resource>;

/// Un `UniqueId` representa un identificador único global para un tipo.
pub type UniqueId = std::any::TypeId;

pub type Weight = i8;

// API *********************************************************************************************

// Funciones y macros útiles.
pub mod util;
// Carga los ajustes de configuración.
pub mod config;
// Ajustes globales.
pub mod global;
// Gestión de trazas y registro de eventos de la aplicación.
pub mod trace;
// HTML en código.
pub mod html;
// Localización.
pub mod locale;
// Soporte a fechas y horas.
pub mod datetime;
// Gestión del servidor y servicios web.
pub mod service;
// Tipos y funciones esenciales para crear acciones, componentes, diseños y paquetes.
pub mod core;
// Respuestas a peticiones web en sus diferentes variantes.
pub mod response;
// Acciones, componentes, diseños y paquetes base.
pub mod base;
// Prepara y ejecuta la aplicación.
pub mod app;

// Prelude de PageTop ******************************************************************************

pub mod prelude;
