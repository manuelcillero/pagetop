//! <div align="center">
//!
//! <h1>Drust</h1>
//!
//! <p>Un Sistema de Gestión de Contenidos (CMS) basado en <strong>PageTop</strong> para compartir tu mundo.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
//! [![Crates.io](https://img.shields.io/crates/v/drust.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/drust)
//! [![Descargas](https://img.shields.io/crates/d/drust.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/drust)
//!
//! </div>
//!
//! # 📌 Sobre PageTop
//!
//! [`PageTop`](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la
//! web clásica combinando SSR (*renderizado en el servidor*), HTML, CSS y JS, para crear soluciones
//! web modulares, extensibles y configurables.

use pagetop::prelude::*;

struct Drust;

impl PackageTrait for Drust {
    fn dependencies(&self) -> Vec<PackageRef> {
        vec![
            // Packages.
            //&pagetop_admin::Admin,
            //&pagetop_user::User,
            //&pagetop_node::Node,

            // Themes.
            //&pagetop_bootsier::Bootsier,
        ]
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&Drust).run()?.await
}
