//! <div align="center">
//!
//! <h1>PageTop SeaORM</h1>
//!
//! <p>Integra SeaORM para trabajar con bases de datos en aplicaciones <strong>PageTop</strong>.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
//! [![Doc API](https://img.shields.io/docsrs/pagetop-seaorm?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-seaorm)
//! [![Crates.io](https://img.shields.io/crates/v/pagetop-seaorm.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-seaorm)
//! [![Descargas](https://img.shields.io/crates/d/pagetop-seaorm.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-seaorm)
//!
//! </div>
//!
//! ## Sobre PageTop
//!
//! [PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la
//! web clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles
//! y configurables, basadas en HTML, CSS y JavaScript.

use pagetop::prelude::*;

use std::sync::LazyLock;

// API *********************************************************************************************

pub mod config;
pub mod db;

// PRELUDE *****************************************************************************************

pub mod prelude {
    pub use crate::db;
}

include_locales!(LOCALES_SEAORM);

/// Implementa [`PackageTrait`] y API específica.
pub struct SeaORM;

impl PackageTrait for SeaORM {
    fn name(&self) -> L10n {
        L10n::t("package_name", &LOCALES_SEAORM)
    }

    fn description(&self) -> L10n {
        L10n::t("package_description", &LOCALES_SEAORM)
    }

    fn init(&self) {
        LazyLock::force(&db::DBCONN);
    }
}
