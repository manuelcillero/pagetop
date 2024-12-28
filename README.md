<div align="center">

<img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/banner.png" />

<h1>PageTop</h1>

<p>Un entorno de desarrollo para crear soluciones web modulares, extensibles y configurables.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
[![Doc API](https://img.shields.io/docsrs/pagetop?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop)
[![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)
[![Descargas](https://img.shields.io/crates/d/pagetop.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop)

</div>

`PageTop` reivindica la esencia de la web clásica usando [Rust](https://www.rust-lang.org/es) para
la creación de soluciones web SSR (*renderizadas en el servidor*) basadas en HTML, CSS y JavaScript.
Ofrece un conjunto de herramientas que los desarrolladores pueden implementar, extender o adaptar
según las necesidades de cada proyecto, incluyendo:

  * **Acciones** (*actions*): alteran la lógica interna de una funcionalidad interceptando su flujo
    de ejecución.
  * **Componentes** (*components*): encapsulan HTML, CSS y JavaScript en unidades funcionales,
    configurables y reutilizables.
  * **Paquetes** (*packages*): añaden, extienden o personalizan funcionalidades usando las APIs de
    `PageTop` o de terceros.
  * **Temas** (*themes*): permiten modificar la apariencia de páginas y componentes sin comprometer
    su funcionalidad.


# ⚡️ Guía rápida

La aplicación más sencilla de `PageTop` se ve así:

```rust#ignore
use pagetop::prelude::*;

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::new().run()?.await
}
```

Por defecto, este código sirve una página web de bienvenida accesible desde un navegador en la
dirección `http://localhost:8088`, siguiendo la configuración predeterminada.

Para personalizar el servicio, puedes crear un paquete de `PageTop` de la siguiente manera:

```rust#ignore
use pagetop::prelude::*;

struct HelloWorld;

impl PackageTrait for HelloWorld {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(hello_world));
    }
}

async fn hello_world(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_component(Html::with(html! { h1 { "Hello world!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).run()?.await
}
```

Este programa implementa un paquete llamado `HelloWorld` que sirve una página web en la ruta raíz
(`/`) mostrando el texto "Hello world!" dentro de un elemento HTML `<h1>`.


# 📂 Repositorio

El código se organiza en un *workspace* con los siguientes subproyectos:

  * **[pagetop](https://github.com/manuelcillero/pagetop/tree/latest/pagetop)**, es la librería
    principal. Reúne algunos de los *crates* más estables y populares del ecosistema Rust para
    proporcionar APIs y recursos para la creación avanzada de soluciones web.

## Auxiliares

  * **[pagetop-build](https://github.com/manuelcillero/pagetop/tree/latest/helpers/pagetop-build)**,
    permite incluir fácilmente archivos estáticos o archivos SCSS compilados directamente en el
    binario de las aplicaciones `PageTop`.

  * **[pagetop-macros](https://github.com/manuelcillero/pagetop/tree/latest/helpers/pagetop-macros)**,
    proporciona una colección de macros que mejoran la experiencia de desarrollo con `PageTop`.

## Paquetes

  * **[pagetop-seaorm](https://github.com/manuelcillero/pagetop/tree/latest/packages/pagetop-seaorm)**,
    integra [SeaORM](https://www.sea-ql.org/SeaORM) para trabajar con bases de datos en aplicaciones
    `PageTop`.

  * **[pagetop-mdbook](https://github.com/manuelcillero/pagetop/tree/latest/packages/pagetop-mdbook)**,
    incluye contenido generado por [mdBook](https://rust-lang.github.io/mdBook/) en aplicaciones
    desarrolladas con `PageTop`.

  * **[pagetop-hljs](https://github.com/manuelcillero/pagetop/tree/latest/packages/pagetop-hljs)**,
    utiliza [HighlightJS](https://highlightjs.org) para mostrar fragmentos de código con resaltado
    de sintaxis con `PageTop`.

## Temas

  * **[pagetop-bootsier](https://github.com/manuelcillero/pagetop/tree/latest/packages/pagetop-bootsier)**,
    tema para `PageTop` que usa [Bootstrap](https://getbootstrap.com) para dar vida a tus diseños
    web.

## Aplicaciones

  * **[drust](https://github.com/manuelcillero/pagetop/tree/latest/drust)**, es una aplicación que
    utiliza `PageTop` para crear un Sistema de Gestión de Contenidos (CMS) que permita construir
    sitios web dinámicos, administrados y configurables.

  * **[website](https://github.com/manuelcillero/pagetop/tree/latest/website)**, es la aplicación
    web creada con el propio entorno `PageTop` para descubrir a la comunidad su ecosistema en
    [pagetop.cillero.es](https://pagetop.cillero.es).


# 🚧 Advertencia

`PageTop` es un proyecto personal que hago por diversión para aprender cosas nuevas. Su API es
inestable y está sujeta a cambios frecuentes. No recomiendo su uso en producción, al menos mientras
no se libere una versión **1.0.0**.


# 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.


# ✨ Contribuciones

Cualquier contribución para añadir al proyecto se considerará automáticamente bajo la doble licencia
indicada arriba (MIT o Apache v2.0), sin términos o condiciones adicionales, tal y como permite la
licencia *Apache v2.0*.
