<div align="center">

<h1>PageTop Macros</h1>

<p>Una colección de macros que mejoran la experiencia de desarrollo con <strong>PageTop</strong>.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
[![Doc API](https://img.shields.io/docsrs/pagetop-macros?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-macros)
[![Crates.io](https://img.shields.io/crates/v/pagetop-macros.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-macros)
[![Descargas](https://img.shields.io/crates/d/pagetop-macros.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-macros)

</div>

# 📦 Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo con convenciones que reivindican la
sencillez de la web clásica aplicando *renderizado en el servidor* (SSR), HTML, CSS y JS.


# 🚧 Advertencia

**PageTop** es un proyecto personal que se encuentra en desarrollo activo. Actualmente su API es
inestable y está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos hasta
que alcance la versión **0.1.0**.


# 🔖 Créditos

Este *crate* incluye una versión adaptada de [maud-macros](https://crates.io/crates/maud_macros)
(versión [0.25.0](https://github.com/lambda-fairy/maud/tree/v0.25.0/maud_macros)) creada por
[Chris Wong](https://crates.io/users/lambda-fairy). También se ha integrado el *crate*
[SmartDefault](https://crates.io/crates/smart_default) (versión 0.7.1), desarrollado por
[Jane Doe](https://crates.io/users/jane-doe), como `AutoDefault` para ampliar el uso de `Default`.

Ambos eliminan la necesidad de referenciar explícitamente `maud` o `smart_default` en el archivo
`Cargo.toml` de cada proyecto.


# 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
