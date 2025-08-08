//! <div align="center">
//!
//! <h1>PageTop Statics</h1>
//!
//! <p>Librería para automatizar la recopilación de recursos estáticos en <strong>PageTop</strong>.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-licencia)
//!
//! </div>
//!
//! ## Sobre PageTop
//!
//! [PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la
//! web clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles
//! y configurables, basadas en HTML, CSS y JavaScript.

#![doc(test(no_crate_inject))]
#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/favicon.ico"
)]
#![allow(clippy::needless_doctest_main)]

/// Resource definition and single module based generation.
pub mod resource;
pub use resource::Resource as StaticResource;

mod resource_dir;
pub use resource_dir::{resource_dir, ResourceDir};

mod resource_files;
pub use resource_files::{ResourceFiles, UriSegmentError};

/// Support for module based generations. Use it for large data sets (more than 128 Mb).
pub mod sets;
