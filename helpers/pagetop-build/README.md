<div align="center">

<h1>PageTop Build</h1>

<p>Prepara un conjunto de archivos estáticos o archivos SCSS compilados para ser incluidos en el binario de un proyecto <strong>PageTop</strong>.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-licencia)
[![Doc API](https://img.shields.io/docsrs/pagetop-build?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-build)
[![Crates.io](https://img.shields.io/crates/v/pagetop-build.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-build)
[![Descargas](https://img.shields.io/crates/d/pagetop-build.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-build)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.


# ⚡️ Guía rápida

Añadir en el archivo `Cargo.toml` del proyecto:

```toml
[build-dependencies]
pagetop-build = { ... }
```

Y crear un archivo `build.rs` a la altura de `Cargo.toml` para indicar cómo se van a incluir los
archivos estáticos o cómo se van a compilar los archivos SCSS para el proyecto. Casos de uso:

## Incluir archivos estáticos desde un directorio

Hay que preparar una carpeta en el proyecto con todos los archivos que se quieren incluir, por
ejemplo `static`, y añadir el siguiente código en `build.rs` para crear el conjunto de recursos:

```rust,no_run
use pagetop_build::StaticFilesBundle;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_dir("./static", None)
        .with_name("guides")
        .build()
}
```

Si es necesario, se puede añadir un filtro para seleccionar archivos específicos de la carpeta, por
ejemplo:

```rust,no_run
use pagetop_build::StaticFilesBundle;
use std::path::Path;

fn main() -> std::io::Result<()> {
    fn only_pdf_files(path: &Path) -> bool {
        // Selecciona únicamente los archivos con extensión `.pdf`.
        path.extension().map_or(false, |ext| ext == "pdf")
    }

    StaticFilesBundle::from_dir("./static", Some(only_pdf_files))
        .with_name("guides")
        .build()
}
```

## Compilar archivos SCSS a CSS

Se puede compilar un archivo SCSS, que podría importar otros a su vez, para preparar un recurso con
el archivo CSS minificado obtenido. Por ejemplo:

```rust,no_run
use pagetop_build::StaticFilesBundle;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_scss("./styles/main.scss", "styles.min.css")
        .with_name("main_styles")
        .build()
}
```

Este código compila el archivo `main.scss` de la carpeta `static` del proyecto, y prepara un recurso
llamado `main_styles` que contiene el archivo `styles.min.css` obtenido.


# 📦 Archivos generados

Cada conjunto de recursos [`StaticFilesBundle`] genera un archivo en el directorio estándar
[OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)
donde se incluye el código necesario para compilar el proyecto. Por ejemplo, para
`with_name("guides")` se genera un archivo llamado `guides.rs`.

No hay ningún problema en generar más de un conjunto de recursos para cada proyecto siempre que se
usen nombres diferentes.

Normalmente no habrá que acceder a estos módulos; sólo declarar el nombre del conjunto de recursos
en [`static_files_service!`](https://docs.rs/pagetop/latest/pagetop/macro.static_files_service.html)
para configurar un servicio web que sirva los archivos desde la ruta indicada. Por ejemplo:

```rust,ignore
use pagetop::prelude::*;

pub struct MyExtension;

impl Extension for MyExtension {
    // Servicio web que publica los recursos de `guides` en `/ruta/a/guides`.
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, guides => "/ruta/a/guides");
    }
}
```


# 🚧 Advertencia

`PageTop` es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.


# 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
