//! <div align="center">
//!
//! <img src="https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/banner.png" />
//!
//! <h1>PageTop</h1>
//!
//! <p>Un entorno de desarrollo para crear soluciones web modulares, extensibles y configurables.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
//!
//! <br>
//! </div>
//!
//! `PageTop` reivindica la esencia de la web clásica usando [Rust](https://www.rust-lang.org/es)
//! para la creación de soluciones web SSR (*renderizadas en el servidor*) basadas en HTML, CSS y
//! JavaScript.
//!
//! # ⚡️ Guía rápida
//!
//! La aplicación más sencilla de `PageTop` se ve así:
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//!
//! #[pagetop::main]
//! async fn main() -> std::io::Result<()> {
//!     Application::new().run()?.await
//! }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

// RE-EXPORTED *************************************************************************************

pub use pagetop_macros::{html, main, test, AutoDefault};

// API *********************************************************************************************

// Funciones y macros útiles.
pub mod util;
// Carga las opciones de configuración.
pub mod config;
// Opciones de configuración globales.
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
// Prepara y ejecuta la aplicación.
pub mod app;

// PRELUDE *****************************************************************************************

pub mod prelude;
