<div align="center">

<h1>PageTop Build</h1>

<p>Genera o prepara archivos estáticos para servirlos o incluirlos en un proyecto <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-build?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-build)
[![Crates.io](https://img.shields.io/crates/v/pagetop-build.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-build)
[![Descargas](https://img.shields.io/crates/d/pagetop-build.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-build)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-build#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

La convención recomendada para extensiones, temas o aplicaciones basadas en **PageTop** es separar
los archivos fuente de los generados siguiendo el patrón `assets/` -> `static/`:

- **`assets/`** - archivos versionados en el repositorio, por ejemplo archivos SCSS, JavaScript de
  terceros, fuentes, etc. Todo lo que hay aquí se sube al repositorio y será la fuente para generar
  el directorio final `static/`.
- **`static/`** - archivos generados en tiempo de compilación a partir de `assets/`. Se añade a
  `.gitignore` y nunca se sube al repositorio.
- **`build.rs`** - orquesta la transformación: genera `static/` desde `assets/` para servirlos o
  incluirlos en el proyecto.

Durante el desarrollo, `static/` existe en disco y los archivos se sirven desde ahí. En producción,
el directorio no existe y los recursos salen del binario. La macro
[`serve_static_files!`](https://docs.rs/pagetop/latest/pagetop/macro.serve_static_files.html)
gestiona esta dualidad de forma transparente.

### Funciones de transformación

Estas funciones se usan en el `build.rs` de cada proyecto para generar `static/` a partir de
`assets/`. Todas crean el directorio padre del destino si no existe y devuelven `io::Result<()>`
para poder propagarse con `?` en caso de error.

- `compile_scss()` - compila un archivo SCSS a CSS minificado.
- `copy_dir()` - copia recursivamente un directorio completo. Útil para copiar todos los archivos de
  `assets/` o de un subdirectorio a `static/` sin transformación.
- `copy_file()` - copia un archivo al destino.
- `copy_file_replacing()` - copia un archivo aplicando una lista de sustituciones de texto en su
  contenido; útil para actualizar referencias internas (p.ej. `sourceMappingURL`) al renombrar
  archivos.
- `minify_js()` - minifica un archivo JavaScript.

### Incluir los archivos estáticos en el proyecto

Una vez generado `static/`, usaremos `StaticFilesBundle` para incluir su contenido en el binario. Se
pueden crear tantos paquetes de recursos como sea necesario, siempre que tengan nombres distintos:

```rust,no_run
use pagetop_build::StaticFilesBundle;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_dir("./static/css", None)
        .with_name("app_css")
        .build()?;
    StaticFilesBundle::from_dir("./static/fonts", None)
        .with_name("app_fonts")
        .build()
}
```

Si es necesario excluir algunos archivos del paquete de recursos (p. ej. los archivos `.map` que no
son necesarios en producción), se puede pasar una función de filtro:

```rust,no_run
use pagetop_build::StaticFilesBundle;
use std::path::Path;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_dir("./static/js", Some(only_js))
        .with_name("app_js")
        .build()
}

fn only_js(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "js")
}
```

Cada paquete de recursos genera un archivo `.rs` en
[OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts).
No es necesario acceder a él directamente: el nombre asignado con `.with_name()` se usa como
identificador en `serve_static_files!` para configurar la ruta del servicio:

```rust,ignore
use pagetop::prelude::*;

pub struct MyExtension;

impl Extension for MyExtension {
    fn configure_router(&self, mut router: Router) -> Router {
        serve_static_files!(router, ["./static/css", app_css] => "/public/css");
        router
    }
}
```

## Ejemplo completo

```rust,no_run
use pagetop_build::StaticFilesBundle;
use pagetop_build::{compile_scss, copy_file, copy_file_replacing, minify_js};
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Regenera `static/` desde cero sólo si hay cambios en `assets/`.
    println!("cargo:rerun-if-changed=assets");
    let _ = std::fs::remove_dir_all("static");

    // Genera `static/` a partir de `assets/`.
    compile_scss("assets/main.scss", "static/css/main.min.css")?;
    copy_file("assets/fonts/icon.woff2", "static/fonts/icon.woff2")?;
    copy_file_replacing(
        "assets/lib.min.js",
        "static/js/app.min.js",
        &[("lib.min.js.map", "app.min.js.map")],
    )?;
    minify_js("assets/shell.js", "static/js/shell.min.js")?;

    // Prepara los paquetes de recursos para incluir en el proyecto.
    StaticFilesBundle::from_dir("./static/css", None).with_name("app_css").build()?;
    StaticFilesBundle::from_dir("./static/js", Some(only_js)).with_name("app_js").build()?;
    StaticFilesBundle::from_dir("./static/fonts", None).with_name("app_fonts").build()
}

// Los `.map` no se incluyen, se servirán desde disco durante el desarrollo.
fn only_js(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "js")
}
```

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
