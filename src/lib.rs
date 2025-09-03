/*!
<div align="center">

<img src="https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/banner.png" />

<h1>PageTop</h1>

<p>Un entorno para el desarrollo de soluciones web modulares, extensibles y configurables.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-licencia)
[![Doc API](https://img.shields.io/docsrs/pagetop?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop)
[![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)
[![Descargas](https://img.shields.io/crates/d/pagetop.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop)

<br>
</div>

PageTop reivindica la esencia de la web clásica usando [Rust](https://www.rust-lang.org/es) para la
creación de soluciones web SSR (*renderizadas en el servidor*) basadas en HTML, CSS y JavaScript.
Ofrece un conjunto de herramientas que los desarrolladores pueden implementar, extender o adaptar
según las necesidades de cada proyecto, incluyendo:

  * **Acciones** (*actions*): alteran la lógica interna de una funcionalidad interceptando su flujo
    de ejecución.
  * **Componentes** (*components*): encapsulan HTML, CSS y JavaScript en unidades funcionales,
    configurables y reutilizables.
  * **Extensiones** (*extensions*): añaden, extienden o personalizan funcionalidades usando las APIs
    de PageTop o de terceros.
  * **Temas** (*themes*): son extensiones que permiten modificar la apariencia de páginas y
    componentes sin comprometer su funcionalidad.


# ⚡️ Guía rápida

La aplicación más sencilla de PageTop se ve así:

```rust,no_run
use pagetop::prelude::*;

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::new().run()?.await
}
```

Este código arranca el servidor de PageTop. Con la configuración por defecto, muestra una página de
bienvenida accesible desde un navegador local en la dirección `http://localhost:8080`.

Para personalizar el servicio, se puede crear una extensión de PageTop de la siguiente manera:

```rust,no_run
use pagetop::prelude::*;

struct HelloWorld;

impl Extension for HelloWorld {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(hello_world));
    }
}

async fn hello_world(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(Some(request))
        .with_component(Html::with(move |_| html! { h1 { "Hello World!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).run()?.await
}
```

Este programa implementa una extensión llamada `HelloWorld` que sirve una página web en la ruta raíz
(`/`) mostrando el texto "Hello world!" dentro de un elemento HTML `<h1>`.


# 🧩 Gestión de Dependencias

Los proyectos que utilizan PageTop gestionan las dependencias con `cargo`, como cualquier otro
proyecto en Rust.

Sin embargo, es fundamental que cada extensión declare explícitamente sus
[dependencias](core::extension::Extension::dependencies), si las tiene, para que PageTop pueda
estructurar e inicializar la aplicación de forma modular.
*/

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/favicon.ico"
)]

// Alias para que las rutas absolutas `::pagetop::...` generadas por las macros funcionen en el
// propio *crate*, en *crates* externos y en *doctests*.
extern crate self as pagetop;

use std::collections::HashMap;
use std::ops::Deref;

// RE-EXPORTED *************************************************************************************

pub use pagetop_macros::{builder_fn, html, main, test, AutoDefault};

pub use pagetop_statics::{resource, StaticResource};

/// Contenedor para un conjunto de recursos embebidos.
#[derive(AutoDefault)]
pub struct StaticResources {
    bundle: HashMap<&'static str, StaticResource>,
}

impl StaticResources {
    /// Crea un contenedor para un conjunto de recursos generado por `build.rs` (consultar
    /// [`pagetop_build`](https://docs.rs/pagetop-build)).
    pub fn new(bundle: HashMap<&'static str, StaticResource>) -> Self {
        Self { bundle }
    }
}

impl Deref for StaticResources {
    type Target = HashMap<&'static str, StaticResource>;

    fn deref(&self) -> &Self::Target {
        &self.bundle
    }
}

/// Identificador único de un tipo estático durante la ejecución de la aplicación.
///
/// **Nota:** El valor es único sólo dentro del proceso actual y cambia en cada compilación.
pub type UniqueId = std::any::TypeId;

/// Representa el peso lógico de una instancia en una colección ordenada por pesos.
///
/// Las instancias con pesos **más bajos**, incluyendo valores negativos (`-128..127`), se situarán
/// antes en la ordenación.
pub type Weight = i8;

// API *********************************************************************************************

// Funciones y macros útiles.
pub mod util;
// Carga las opciones de configuración.
pub mod config;
// Opciones de configuración globales.
pub mod global;
// Gestión de trazas y registro de eventos de la aplicación.
pub mod trace;
// HTML en código.
pub mod html;
// Localización.
pub mod locale;
// Soporte a fechas y horas.
pub mod datetime;
// Tipos y funciones esenciales para crear acciones, componentes, extensiones y temas.
pub mod core;
// Respuestas a peticiones web en sus diferentes formatos.
pub mod response;
// Gestión del servidor y servicios web.
pub mod service;
// Reúne acciones, componentes, extensiones y temas predefinidos.
pub mod base;
// Prepara y ejecuta la aplicación.
pub mod app;

// PRELUDE *****************************************************************************************

pub mod prelude;
