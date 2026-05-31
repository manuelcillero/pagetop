/*!
<div align="center">

<h1>PageTop Build</h1>

<p>Prepara un conjunto de archivos estáticos o archivos SCSS compilados para ser incluidos en el binario de un proyecto <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-build?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-build)
[![Crates.io](https://img.shields.io/crates/v/pagetop-build.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-build)
[![Descargas](https://img.shields.io/crates/d/pagetop-build.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-build)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-build#licencia)

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
en [`serve_static_files!`](https://docs.rs/pagetop/latest/pagetop/macro.serve_static_files.html)
para configurar un servicio web que sirva los archivos desde la ruta indicada. Por ejemplo:

```rust,ignore
use pagetop::prelude::*;

pub struct MyExtension;

impl Extension for MyExtension {
    /// Registra los recursos de `guides` en el router bajo `/ruta/a/guides`.
    fn configure_router(&self, mut router: Router) -> Router {
        serve_static_files!(router, [guides] => "/ruta/a/guides");
        router
    }
}
```
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/favicon.ico"
)]

use grass::{Options, OutputStyle, from_path};
use pagetop_statics::{ResourceDir, resource_dir};

use std::fs::{File, create_dir_all, remove_dir_all};
use std::io::Write;
use std::path::Path;

/// Prepara un conjunto de recursos para ser incluidos en el binario del proyecto.
pub struct StaticFilesBundle {
    resource_dir: ResourceDir,
}

impl StaticFilesBundle {
    /// Prepara el conjunto de recursos con los archivos de un directorio. Opcionalmente se puede
    /// aplicar un filtro para seleccionar un subconjunto de los archivos.
    ///
    /// # Argumentos
    ///
    /// * `dir` - Directorio que contiene los archivos.
    /// * `filter` - Una función opcional para aceptar o no un archivo según su ruta.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// use pagetop_build::StaticFilesBundle;
    /// use std::path::Path;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     fn only_images(path: &Path) -> bool {
    ///         matches!(
    ///             path.extension().and_then(|ext| ext.to_str()),
    ///             Some("jpg" | "png" | "gif")
    ///         )
    ///     }
    ///
    ///     StaticFilesBundle::from_dir("./static", Some(only_images))
    ///         .with_name("images")
    ///         .build()
    /// }
    /// ```
    pub fn from_dir<P>(dir: P, filter: Option<fn(&Path) -> bool>) -> Self
    where
        P: AsRef<Path>,
    {
        let dir_path = dir.as_ref();
        let dir_str = dir_path.to_str().unwrap_or_else(|| {
            panic!(
                "Resource directory path is not valid UTF-8: {}",
                dir_path.display()
            );
        });

        let mut resource_dir = resource_dir(dir_str);

        // Aplica el filtro si está definido.
        if let Some(f) = filter {
            resource_dir.with_filter(f);
        }

        // Identifica el directorio temporal de recursos.
        StaticFilesBundle { resource_dir }
    }

    /// Prepara un recurso CSS minimizado a partir de la compilación de un archivo SCSS (que puede a
    /// su vez importar otros archivos SCSS).
    ///
    /// # Argumentos
    ///
    /// * `path` - Archivo SCSS a compilar.
    /// * `target_name` - Nombre para el archivo CSS.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// use pagetop_build::StaticFilesBundle;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     StaticFilesBundle::from_scss("./bootstrap/scss/main.scss", "bootstrap.min.css")
    ///         .with_name("bootstrap_css")
    ///         .build()
    /// }
    /// ```
    pub fn from_scss<P>(path: P, target_name: &str) -> Self
    where
        P: AsRef<Path>,
    {
        // Crea un directorio temporal para el archivo CSS.
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let temp_dir = Path::new(&out_dir).join("from_scss_files");

        // Limpia el directorio temporal de ejecuciones previas, si existe.
        if temp_dir.exists() {
            remove_dir_all(&temp_dir).unwrap_or_else(|e| {
                panic!(
                    "Failed to clean temporary directory `{}`: {e}",
                    temp_dir.display()
                );
            });
        }
        create_dir_all(&temp_dir).unwrap_or_else(|e| {
            panic!(
                "Failed to create temporary directory `{}`: {e}",
                temp_dir.display()
            );
        });

        // Compila SCSS a CSS.
        let css_content = from_path(
            path.as_ref(),
            &Options::default().style(OutputStyle::Compressed),
        )
        .unwrap_or_else(|e| {
            panic!(
                "Failed to compile SCSS file `{}`: {e}",
                path.as_ref().display(),
            )
        });

        // Guarda el archivo CSS compilado en el directorio temporal.
        let css_path = temp_dir.join(target_name);
        File::create(&css_path)
            .unwrap_or_else(|_| panic!("Failed to create CSS file `{}`", css_path.display()))
            .write_all(css_content.as_bytes())
            .unwrap_or_else(|_| panic!("Failed to write CSS content to `{}`", css_path.display()));

        // Identifica el directorio temporal de recursos.
        StaticFilesBundle {
            resource_dir: resource_dir(temp_dir.to_str().unwrap()),
        }
    }

    /// Asigna un nombre al conjunto de recursos.
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let filename = Path::new(&out_dir).join(format!("{name}.rs"));
        self.resource_dir.with_generated_filename(filename);
        self.resource_dir.with_module_name(format!("bundle_{name}"));
        self.resource_dir.with_generated_fn(name);
        self
    }

    /// Contruye finalmente el conjunto de recursos para incluir en el binario de la aplicación.
    pub fn build(self) -> std::io::Result<()> {
        self.resource_dir.build()
    }
}
