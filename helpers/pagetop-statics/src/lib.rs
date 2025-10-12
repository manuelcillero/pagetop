/*!
<div align="center">

<h1>PageTop Statics</h1>

<p>Librería para automatizar la recopilación de recursos estáticos en <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-statics?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-statics)
[![Crates.io](https://img.shields.io/crates/v/pagetop-statics.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-statics)
[![Descargas](https://img.shields.io/crates/d/pagetop-statics.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-statics)
 ![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Descripción general

Esta librería permite incluir archivos estáticos en el ejecutable de las aplicaciones PageTop para
servirlos de forma eficiente vía web, con detección de cambios que optimizan el tiempo de
compilación.

## Créditos

Para ello, adapta el código de los *crates* [static-files](https://crates.io/crates/static_files)
(versión [0.2.5](https://github.com/static-files-rs/static-files/tree/v0.2.5)) y
[actix-web-static-files](https://crates.io/crates/actix_web_static_files) (versión
[4.0.1](https://github.com/kilork/actix-web-static-files/tree/v4.0.1)), desarrollados ambos por
[Alexander Korolev](https://crates.io/users/kilork).

Estas implementaciones se integran en PageTop para evitar que cada proyecto tenga que declarar
`static-files` manualmente como dependencia en su `Cargo.toml`.
*/

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
