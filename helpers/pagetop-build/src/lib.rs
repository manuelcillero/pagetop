/*!
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
  contenido; útil para actualizar referencias internas (p. ej. `sourceMappingURL`) al renombrar
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

#[async_trait]
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
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/assets/favicon.ico"
)]

use grass::{Options, OutputStyle, from_path};
use minify_js::{Session, TopLevelMode, minify};
use pagetop_statics::resource_dir;

use std::fs::{File, copy as fs_copy, create_dir_all, read_dir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// **< StaticFilesBundle >**************************************************************************

/// Prepara un paquete de recursos para incluir en el binario del proyecto.
pub struct StaticFilesBundle {
    dir: PathBuf,
    filter: Option<fn(&Path) -> bool>,
    name: Option<String>,
}

impl StaticFilesBundle {
    /// Crea el paquete de recursos con los archivos del directorio indicado.
    ///
    /// # Argumentos
    ///
    /// * `dir` - Ruta al directorio con los archivos a incluir, normalmente `static/` o un
    ///   directorio dentro de este.
    /// * `filter` - Función opcional para seleccionar qué archivos incluir en el paquete.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// use pagetop_build::StaticFilesBundle;
    /// use std::path::Path;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     StaticFilesBundle::from_dir("./static", Some(only_images))
    ///         .with_name("images")
    ///         .build()
    /// }
    ///
    /// fn only_images(path: &Path) -> bool {
    ///     matches!(
    ///         path.extension().and_then(|ext| ext.to_str()),
    ///         Some("jpg" | "png" | "gif")
    ///     )
    /// }
    /// ```
    pub fn from_dir<P>(dir: P, filter: Option<fn(&Path) -> bool>) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            dir: dir.as_ref().to_path_buf(),
            filter,
            name: None,
        }
    }

    /// Asigna un nombre al paquete de recursos.
    ///
    /// El nombre debe ser un identificador Rust válido que se convertirá en nombre del módulo y de
    /// la función del archivo `.rs` generado en `OUT_DIR`. Si no se llama a este método, el nombre
    /// por defecto será `"bundle"`.
    ///
    /// Este nombre es el que hay que declarar en
    /// [`serve_static_files!`](https://docs.rs/pagetop/latest/pagetop/macro.serve_static_files.html)
    /// para configurar la ruta del servicio:
    ///
    /// ```rust,ignore
    /// serve_static_files!(router, ["./static/css", app_css] => "/public/css");
    /// //                                           ^^^^^^^
    /// //                                           debe coincidir con .with_name("app_css")
    /// ```
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    /// Genera el archivo `.rs` en `OUT_DIR` para incluir los recursos del directorio en el binario.
    pub fn build(self) -> std::io::Result<()> {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let name = self.name.as_deref().unwrap_or("bundle");

        let mut rd = resource_dir(&self.dir);
        if let Some(f) = self.filter {
            rd.with_filter(f);
        }

        let generated_filename = PathBuf::from(&out_dir).join(format!("{name}.rs"));
        rd.with_generated_filename(generated_filename);
        rd.with_module_name(format!("bundle_{name}"));
        rd.with_generated_fn(name);
        rd.build()
    }
}

// **< compile_scss / copy_dir / copy_file / copy_file_replacing / minify_js >**********************

/// Compila un archivo SCSS a CSS minificado y lo escribe en la ruta de destino.
///
/// Crea el directorio padre del destino si no existe.
///
/// # Ejemplo
///
/// ```rust,no_run
/// fn main() -> std::io::Result<()> {
///     pagetop_build::compile_scss("assets/main.scss", "static/css/main.min.css")
/// }
/// ```
pub fn compile_scss<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    if let Some(parent) = dst.parent() {
        create_dir_all(parent)?;
    }

    let options = Options::default().style(OutputStyle::Compressed);
    let css = from_path(src, &options)
        .map_err(|e| io::Error::other(format!("failed to compile `{}`: {e}", src.display())))?;
    File::create(dst)?.write_all(css.as_bytes())
}

/// Copia recursivamente el contenido de un directorio a otro destino.
///
/// Crea el directorio destino y todos los subdirectorios necesarios.
///
/// # Ejemplo
///
/// ```rust,no_run
/// fn main() -> std::io::Result<()> {
///     pagetop_build::copy_dir("assets", "static")
/// }
/// ```
pub fn copy_dir<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();
    create_dir_all(dst)?;
    for entry in read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs_copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Copia un archivo a su destino.
///
/// Crea el directorio padre del destino si no existe.
///
/// # Ejemplo
///
/// ```rust,no_run
/// fn main() -> std::io::Result<()> {
///     pagetop_build::copy_file("assets/fonts/icon.woff2", "static/fonts/icon.woff2")
/// }
/// ```
pub fn copy_file<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    if let Some(parent) = dst.parent() {
        create_dir_all(parent)?;
    }

    fs_copy(src, dst)?;
    Ok(())
}

/// Copia un archivo a su destino con una lista de sustituciones de texto en su contenido.
///
/// El archivo fuente se lee como texto UTF-8; no debe usarse con archivos binarios. Las
/// sustituciones de texto se aplican en orden y de forma encadenada: el resultado de cada
/// sustitución puede ser entrada de la siguiente.
///
/// Crea el directorio padre del destino si no existe.
///
/// # Ejemplo
///
/// ```rust,no_run
/// fn main() -> std::io::Result<()> {
///     pagetop_build::copy_file_replacing(
///         "assets/adminlte.min.js",
///         "static/js/myapp.min.js",
///         &[("adminlte.min.js.map", "myapp.min.js.map")],
///     )
/// }
/// ```
pub fn copy_file_replacing<P, Q>(src: P, dst: Q, replacements: &[(&str, &str)]) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    if let Some(parent) = dst.parent() {
        create_dir_all(parent)?;
    }

    let content = std::fs::read_to_string(src)?;
    let patched = replacements
        .iter()
        .fold(content, |acc, (old, new)| acc.replace(old, new));
    File::create(dst)?.write_all(patched.as_bytes())
}

/// Minifica un archivo JavaScript y lo escribe en la ruta de destino.
///
/// El archivo se procesa en modo de ámbito global (`TopLevelMode::Global`), adecuado para scripts
/// sin `import`/`export`. Los archivos con sintaxis de módulo ES deben procesarse con
/// `TopLevelMode::Module`, que el *crate* subyacente (`minify-js`) también soporta pero esta
/// función no expone actualmente.
///
/// Crea el directorio padre del destino si no existe.
///
/// # Ejemplo
///
/// ```rust,no_run
/// fn main() -> std::io::Result<()> {
///     pagetop_build::minify_js("assets/shell.js", "static/js/shell.min.js")
/// }
/// ```
pub fn minify_js<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    if let Some(parent) = dst.parent() {
        create_dir_all(parent)?;
    }

    let source = std::fs::read(src)?;
    let session = Session::new();
    let mut output = Vec::new();
    minify(&session, TopLevelMode::Global, &source, &mut output)
        .map_err(|e| io::Error::other(format!("failed to minify `{}`: {e:?}", src.display())))?;
    File::create(dst)?.write_all(&output)
}
