<div align="center">

<h1>PageTop Minimal</h1>

<p>Reúne un conjunto mínimo de macros para mejorar el formato y la eficiencia de operaciones básicas en <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-minimal?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-minimal)
[![Crates.io](https://img.shields.io/crates/v/pagetop-minimal.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-minimal)
[![Descargas](https://img.shields.io/crates/d/pagetop-minimal.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-minimal)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-minimal#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Descripción general

Este *crate* proporciona un conjunto básico de macros que se integran en las utilidades de PageTop
para optimizar operaciones habituales relacionadas con la composición estructurada de texto, la
concatenación de cadenas y el uso rápido de colecciones clave-valor.

## Créditos

Las macros para texto multilínea **`indoc!`**, **`formatdoc!`** y **`concatdoc!`** se reexportan del
*crate* [indoc](https://crates.io/crates/indoc) de [David Tolnay](https://crates.io/users/dtolnay).

Las macros para la concatenación de cadenas **`join!`** y **`join_pair!`** se apoyan internamente en
el *crate* [concat-string](https://crates.io/crates/concat_string), desarrollado por
[FaultyRAM](https://crates.io/users/FaultyRAM), para evitar el formato de cadenas cuando la
eficiencia pueda ser relevante.

La macro para generar identificadores dinámicos **`paste!`** se reexporta del *crate*
[pastey](https://crates.io/crates/pastey), una implementación avanzada y soportada del popular
`paste!` de [David Tolnay](https://crates.io/users/dtolnay).

## Advertencia

**PageTop** es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.

## Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
