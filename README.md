<div align="center">

<img src="https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/banner.png" />

<h1>PageTop</h1>

<p>Un entorno para el desarrollo de soluciones web modulares, extensibles y configurables.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop)
[![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)
[![Descargas](https://img.shields.io/crates/d/pagetop.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop#licencia)

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


## ⚡️ Guía rápida

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
    Page::new(request)
        .add_child(Html::with(|_| html! { h1 { "Hello World!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).run()?.await
}
```

Este programa implementa una extensión llamada `HelloWorld` que sirve una página web en la ruta raíz
(`/`) mostrando el texto "Hello world!" dentro de un elemento HTML `<h1>`.


## 📂 Proyecto

El código se organiza en un *workspace* donde actualmente se incluyen los siguientes subproyectos:

  * **[pagetop](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/src)**, con el código
    fuente de la librería principal. Reúne algunos de los *crates* más estables y populares del
    ecosistema Rust para proporcionar APIs y recursos para la creación avanzada de soluciones web.

### Auxiliares

  * **[pagetop-build](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-build)**,
    prepara los archivos estáticos o archivos SCSS compilados para incluirlos en el binario de las
    aplicaciones PageTop durante la compilación de los ejecutables.

  * **[pagetop-macros](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-macros)**,
    proporciona una colección de macros procedurales que mejoran la experiencia de desarrollo con
    PageTop.

  * **[pagetop-minimal](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-minimal)**,
    ofrece macros declarativas esenciales para optimizar tareas comunes como la composición de
    texto, la concatenación de cadenas y el manejo de colecciones clave-valor.

  * **[pagetop-statics](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-statics)**,
    permite incluir archivos estáticos en el ejecutable de las aplicaciones PageTop para servirlos
    de forma eficiente, con detección de cambios que optimizan el tiempo de compilación.

### Extensiones

  * **[pagetop-aliner](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-aliner)**,
    es un tema para demos y pruebas que muestra esquemáticamente la composición de las páginas HTML.

  * **[pagetop-bootsier](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-bootsier)**,
    tema basado en [Bootstrap](https://getbootstrap.com) para integrar su catálogo de estilos y
    componentes flexibles.


## 🧪 Pruebas

Para simplificar el flujo de trabajo, el repositorio incluye varios **alias de Cargo** declarados en
`.cargo/config.toml`. Basta con ejecutarlos desde la raíz del proyecto:

| Comando | Descripción |
| ------- | ----------- |
| `cargo ts` | Ejecuta los tests de `pagetop` (*unit + integration*) con la *feature* `testing`. |
| `cargo ts --test util` | Lanza sólo las pruebas de integración del módulo `util`. |
| `cargo ts --doc locale` | Lanza las pruebas de la documentación del módulo `locale`. |
| `cargo tw` | Ejecuta los tests de **todos los paquetes** del *workspace*. |

> **Nota**
> Estos alias ya compilan con la configuración adecuada. No requieren `--no-default-features`.
> Si quieres **activar** las trazas del registro de eventos entonces usa simplemente `cargo test`.


## 🚧 Advertencia

**PageTop** es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.


## 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.


## ✨ Contribuir

PageTop mantiene **un único repositorio oficial**:

  * **Repositorio oficial:** https://git.cillero.es/manuelcillero/pagetop
  * **Repositorio espejo:** https://github.com/manuelcillero/pagetop

El repositorio de GitHub actúa como espejo y punto de entrada para:

  * dar mayor visibilidad al proyecto,
  * facilitar la participación de la comunidad,
  * centralizar *issues* y *pull requests* externas.

Aunque GitHub permite abrir *pull requests*, **la integración del código se realiza únicamente en el
repositorio oficial**. El repositorio de GitHub se sincroniza posteriormente para reflejar el mismo
estado.

En todos los casos, se respeta la **autoría original** de las contribuciones integradas, tanto en el
historial como en la documentación asociada al cambio.

Para conocer el proceso completo de participación, revisión e integración de cambios, consulta el
archivo [`CONTRIBUTING.md`](CONTRIBUTING.md).

Cualquier contribución para añadir al proyecto se considerará automáticamente bajo la doble licencia
indicada arriba (MIT o Apache v2.0), sin términos o condiciones adicionales, tal y como permite la
licencia *Apache v2.0*.
