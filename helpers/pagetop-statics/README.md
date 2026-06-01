<div align="center">

<h1>PageTop Statics</h1>

<p>Librería para automatizar la recopilación de recursos estáticos en <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-statics?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-statics)
[![Crates.io](https://img.shields.io/crates/v/pagetop-statics.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-statics)
[![Descargas](https://img.shields.io/crates/d/pagetop-statics.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-statics)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-statics#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Descripción general

Este *crate* permite incluir archivos estáticos en el ejecutable de las aplicaciones PageTop para
servirlos de forma eficiente vía web, con detección de cambios que optimizan el tiempo de
compilación.

## Créditos

Para ello, adapta el código de [static-files](https://crates.io/crates/static_files) (versión
[0.2.5](https://github.com/static-files-rs/static-files/tree/v0.2.5)) desarrollado por
[Alexander Korolev](https://crates.io/users/kilork), bajo licencia MIT/Apache 2.0. La implementación
se integra en PageTop para evitar que cada proyecto tenga que declarar `static-files` manualmente
como dependencia en su `Cargo.toml`.


## 🚧 Advertencia

**PageTop** es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.


## 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
