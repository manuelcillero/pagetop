//! <div align="center">
//!
//! <h1>PageTop Build</h1>
//!
//! <p>Incluye fácilmente archivos estáticos o archivos SCSS compilados directamente en el binario de tus aplicaciones <strong>PageTop</strong>.</p>
//!
//! [![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
//! [![Doc API](https://img.shields.io/docsrs/pagetop-build?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-build)
//! [![Crates.io](https://img.shields.io/crates/v/pagetop-build.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-build)
//! [![Descargas](https://img.shields.io/crates/d/pagetop-build.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-build)
//!
//! </div>
//!
//! # 📌 Sobre PageTop
//!
//! [`PageTop`](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la sencillez de
//! la web clásica combinando SSR (*renderizado en el servidor*), HTML, CSS y JS, para crear
//! soluciones web modulares, extensibles y configurables.
//!
//! # ⚡️ Guía rápida
//!
//! Añade en el archivo `Cargo.toml` de tu aplicación:
//!
//! ```toml
//! [build-dependencies]
//! pagetop-build = { ... }
//! ```
//!
//! Luego crea un archivo `build.rs` para definir cómo se van a incluir los archivos estáticos o
//! archivos SCSS en tu aplicación. Casos de uso:
//!
//! ## Incluir archivos estáticos desde un directorio
//!
//! Prepara una carpeta en tu proyecto con todos los archivos que deseas incluir, por ejemplo
//! `static`, y añade el siguiente código a tu archivo `build.rs` para crear tu conjunto de
//! recursos:
//!
//! ```rust#ignore
//! use pagetop_build::StaticFilesBundle;
//!
//! fn main() -> std::io::Result<()> {
//!     StaticFilesBundle::from_dir("./static", None)
//!         .with_name("guides")
//!         .build()
//! }
//! ```
//!
//! Si es necesario, puedes añadir un filtro para seleccionar archivos específicos de la carpeta:
//!
//! ```rust#ignore
//! use pagetop_build::StaticFilesBundle;
//! use std::path::Path;
//!
//! fn main() -> std::io::Result<()> {
//!     fn only_pdf_files(path: &Path) -> bool {
//!         // Include only files with the `.pdf` extension.
//!         path.extension().map_or(false, |ext| ext == "pdf")
//!     }
//!
//!     StaticFilesBundle::from_dir("./static", Some(only_pdf_files))
//!         .with_name("guides")
//!         .build()
//! }
//! ```
//!
//! ## Compilar archivos SCSS a CSS
//!
//! Puedes compilar un archivo SCSS, que podría importar otros a su vez, para preparar un conjunto
//! de recursos con el archivo CSS obtenido. Por ejemplo:
//!
//! ```rust#ignore
//! use pagetop_build::StaticFilesBundle;
//!
//! fn main() -> std::io::Result<()> {
//!     StaticFilesBundle::from_scss("./styles/main.scss", "styles.css")
//!         .with_name("main_styles")
//!         .build()
//! }
//! ```
//!
//! Este código compila el archivo `main.scss` de la carpeta `static` del proyecto, en un archivo
//! `styles.css` que se preparará como un conjunto de recursos llamado `main_styles`.
//!
//!
//! # 📦 Módulos generados
//!
//! Cada conjunto de recursos [`StaticFilesBundle`] genera un archivo en el directorio estándar
//! [OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)
//! donde se incluyen los recursos necesarios para la compilación. Por ejemplo, para
//! `with_name("guides")` se crea un archivo llamado `guides.rs`.
//!
//! Ten en cuenta que puedes generar más de un conjunto de recursos para usar en tu proyecto.
//!
//! Normalmente no necesitarás acceder directamente a este archivo; sólo inclúyelo en tu proyecto
//! con [`include_files!`](https://docs.rs/pagetop/latest/pagetop/macro.include_files.html), y luego
//! configura un servicio web usando [`include_files_service!`](https://docs.rs/pagetop/latest/pagetop/macro.include_files_service.html)
//! para servir tu conjunto de recursos desde la ruta indicada:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! include_files!(guides);
//!
//! pub struct MyPackage;
//!
//! impl PackageTrait for MyPackage {
//!     // Configures a web service to expose the `guides` resources at `/route/to/guides`.
//!     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
//!         include_files_service!(scfg, guides => "/route/to/guides");
//!     }
//! }
//! ```
//!
//! También podrías acceder a tu conjunto de recursos declarando un `HashMap` estático global:
//!
//! ```rust#ignore
//! use pagetop::prelude::*;
//!
//! // Declares a static `HashMap` for accessing included resources as key-value pairs.
//! include_files!(HM_GUIDES => guides);
//! ```

use grass::{from_path, Options, OutputStyle};
use static_files::{resource_dir, ResourceDir};

use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::Path;

/// Genera el conjunto de recursos a incluir en el binario de la aplicación utilizando
/// [static_files](https://docs.rs/static-files/latest/static_files/).
pub struct StaticFilesBundle {
    resource_dir: ResourceDir,
}

impl StaticFilesBundle {
    /// Prepara el conjunto de recursos con los archivos de un directorio en el que se puede aplicar
    /// opcionalmente un filtro para seleccionar los adecuados.
    ///
    /// # Argumentos
    ///
    /// * `dir` - Directorio que contiene los archivos.
    /// * `filter` - Una función opcional para filtrar los archivos o directorios a incluir.
    pub fn from_dir(dir: &'static str, filter: Option<fn(p: &Path) -> bool>) -> Self {
        let mut resource_dir = resource_dir(dir);

        // Aplica el filtro si está definido.
        if let Some(f) = filter {
            resource_dir.with_filter(f);
        }

        // Identifica el directorio temporal de recursos.
        StaticFilesBundle { resource_dir }
    }

    /// Prepara un recurso CSS a partir de la compilación de un archivo SCSS que puede importar
    /// otros a su vez.
    ///
    /// # Argumentos
    ///
    /// * `path` - Archivo SCSS a compilar.
    /// * `target_name` - Nombre para el archivo CSS.
    ///
    /// # Panics
    ///
    /// Esta función generará un *panic* si:
    ///
    /// * La variable de entorno `OUT_DIR` no está asignada.
    /// * No se puede crear un directorio temporal en `OUT_DIR`.
    /// * No se puede compilar el archivo SCSS debido a errores de sintaxis, faltan dependencias o
    ///   rutas de importación necesarias para la compilación.
    /// * No se puede crear el archivo CSS en el directorio temporal debido a un `target_name` no
    ///   válido o permisos insuficientes.
    /// * La función falla al escribir el contenido CSS compilado en el archivo.
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
            .expect(&format!(
                "Failed to create CSS file `{}`",
                css_path.display()
            ))
            .write_all(css_content.as_bytes())
            .expect(&format!(
                "Failed to write CSS content to `{}`",
                css_path.display()
            ));

        // Identifica el directorio temporal de recursos.
        StaticFilesBundle {
            resource_dir: resource_dir(temp_dir.to_str().unwrap()),
        }
    }

    /// Asigna un nombre al conjunto de recursos.
    ///
    /// # Panics
    ///
    /// Esta función generará un *panic* si la variable de entorno `OUT_DIR` no está asignada.
    pub fn with_name(mut self, name: &'static str) -> Self {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let filename = Path::new(&out_dir).join(format!("{name}.rs"));
        self.resource_dir.with_generated_filename(filename);
        self.resource_dir.with_module_name(format!("bundle_{name}"));
        self.resource_dir.with_generated_fn(name);
        self
    }

    /// Crea el conjunto de recursos para incluir en el binario de la aplicación.
    ///
    /// # Errores
    ///
    /// Esta función devolverá un error si ocurre algún problema con las operaciones de E/S,
    /// como fallos al leer o escribir en un archivo.
    pub fn build(self) -> std::io::Result<()> {
        self.resource_dir.build()
    }
}
