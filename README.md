<div align="center">

<img src="https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/banner.png" />

<h1>PageTop</h1>

<p>Entorno de desarrollo para crear soluciones web modulares, extensibles y configurables.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
[![Doc API](https://img.shields.io/docsrs/pagetop?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop)
[![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)
[![Descargas](https://img.shields.io/crates/d/pagetop.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop)

</div>

**PageTop** reúne algunos de los *crates* más estables y populares del ecosistema Rust para
proporcionar un conjunto completo de funcionalidades que pueden extenderse y adaptarse a las
necesidades específicas de cada aplicación web.

PageTop reivindica la sencillez de la web clásica aplicando *renderizado en el servidor* (SSR),
HTML, CSS y JS, mediante acciones, componentes, diseños y paquetes:

  * **Acciones** (*actions*). Las funcionalidades que incorporen acciones en su lógica de programa
    estarán proporcionando a los desarrolladores herramientas para alterar su comportamiento interno
    interceptando su flujo de ejecución.
  * **Componentes** (*components*). Encapsulan HTML, CSS y JavaScript en unidades funcionales,
    configurables y bien definidas.
  * **Diseños** (*layouts*). Permiten a los desarrolladores modificar la apariencia de páginas y
    componentes sin afectar a su funcionalidad.
  * **Paquetes** (*packages*). Extienden o personalizan funcionalidades existentes interactuando con
    las APIs de PageTop o de paquetes de terceros.


# ⚡️ Inicio rápido

La aplicación más sencilla de PageTop se ve así:

```rust
use pagetop::prelude::*;

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::new().run()?.await
}
```

Proporciona una página de bienvenida en `http://localhost:8088` según la configuración predefinida.
Para personalizar el servicio puedes crear un paquete de PageTop:

```rust
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

Este programa prepara un paquete personalizado llamado `HelloWorld` que sirve una página web en la
ruta raíz (`/`) mostrando el mensaje "Hello world!" en un elemento HTML `<h1>`.


# 📂 Crates de ayuda

  * [pagetop-macros](https://github.com/manuelcillero/pagetop/tree/latest/helpers/pagetop-macros):
    Proporciona una colección de macros que mejoran la experiencia de desarrollo con PageTop.

  * [pagetop-build](https://github.com/manuelcillero/pagetop/tree/latest/helpers/pagetop-build):
    Permite incluir fácilmente archivos estáticos o archivos SCSS compilados, directamente en el
    binario de las aplicaciones PageTop.


# 🚧 Advertencia

**PageTop** es un proyecto personal que hago por diversión para aprender cosas nuevas. Su API es
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
