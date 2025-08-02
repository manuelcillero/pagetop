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
#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/favicon.ico"
)]

// Alias para que las rutas absolutas `::pagetop::...` generadas por las macros funcionen en el
// propio *crate*, en *crates* externos y en *doctests*.
extern crate self as pagetop;

use std::collections::HashMap;
use std::ops::Deref;

// RE-EXPORTED *************************************************************************************

pub use pagetop_macros::{builder_fn, html, main, test, AutoDefault};

/// Conjunto de recursos asociados a `$STATIC` en [`include_files!`](crate::include_files).
pub struct StaticResources {
    bundle: HashMap<&'static str, static_files::Resource>,
}

impl StaticResources {
    /// Crea un contenedor para un conjunto de recursos generado por `build.rs` (consultar
    /// [`pagetop_build`](https://docs.rs/pagetop-build)).
    pub fn new(bundle: HashMap<&'static str, static_files::Resource>) -> Self {
        Self { bundle }
    }
}

impl Deref for StaticResources {
    type Target = HashMap<&'static str, static_files::Resource>;

    fn deref(&self) -> &Self::Target {
        &self.bundle
    }
}

/// Identificador único de un tipo estático durante la ejecución de la aplicación.
///
/// **Nota:** El valor es único sólo dentro del proceso actual y cambia en cada compilación.
pub type UniqueId = std::any::TypeId;

/// Representa el peso lógico de una instancia en una colección ordenada por pesos.
///
/// Las instancias con pesos **más bajos**, incluyendo valores negativos (`-128..127`), se situarán
/// antes en la ordenación.
pub type Weight = i8;

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
// Tipos y funciones esenciales para crear acciones, componentes, extensiones y temas.
pub mod core;
// Respuestas a peticiones web en sus diferentes formatos.
pub mod response;
// Gestión del servidor y servicios web.
pub mod service;
// Reúne acciones, componentes, extensiones y temas predefinidos.
pub mod base;
// Prepara y ejecuta la aplicación.
pub mod app;

// PRELUDE *****************************************************************************************

pub mod prelude;
