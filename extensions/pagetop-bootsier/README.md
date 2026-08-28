<div align="center">

<h1>PageTop Bootsier</h1>

<p>Tema de <strong>PageTop</strong> basado en Bootstrap para aplicar su catálogo de estilos y componentes flexibles.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-bootsier?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-bootsier)
[![Crates.io](https://img.shields.io/crates/v/pagetop-bootsier.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-bootsier)
[![Descargas](https://img.shields.io/crates/d/pagetop-bootsier.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-bootsier)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-bootsier#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

Igual que con otras extensiones, **añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-bootsier = { ... }
```

**Declara la extensión** en tu aplicación (o extensión que la requiera). Recuerda que el orden en
`dependencies()` determina la prioridad relativa frente a las otras extensiones:

```rust,no_run
use pagetop::prelude::*;

struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            // ...
            &pagetop_bootsier::Bootsier,
            // ...
        ]
    }
}
```

Y **selecciona el tema en la configuración** de la aplicación:

```toml
[app]
theme = "Bootsier"
```

o **fuerza el tema por código** en una página concreta:

```rust,no_run
use pagetop::prelude::*;
use pagetop_bootsier::Bootsier;

async fn homepage(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_theme(&Bootsier)
        .with_child(
            Block::new()
                .with_title(Lc::l("sample_title"))
                .with_child(Html::with(|cx| html! {
                    p { (Lc::l("sample_content").using(cx)) }
                })),
        )
        .render().await
}
```

## Plantillas

Bootsier ofrece dos plantillas (`BootsierTemplates`): `Standard`, con cabecera, contenido y pie,
que es la plantilla por defecto de cualquier página; y `Admin`, con la shell completa de
AdminLTE 4 (barra superior + barra lateral + área de contenido), que se activa creando la página
con `Page::admin()` en lugar de `Page::new()`.

```rust,no_run
use pagetop::prelude::*;

async fn about(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| html! {
            h1 { "Sobre nosotros" }
            p { "Texto de presentación." }
        }))
        .render()
}
```

## Barra lateral

Registra elementos en `BootsierRegion::Sidebar` para poblar la barra lateral de la shell. Los
elementos esperados son `theme::sidebar::Item` y `theme::sidebar::Section`.

De forma **global** (visibles en todas las páginas de administración):

```rust,no_run
use pagetop::prelude::*;
use pagetop_bootsier::theme::{BootsierRegion, sidebar};

fn register_navigation() {
    InRegion::Global(&BootsierRegion::Sidebar)
        .add(sidebar::Section::titled(Lc::n("Administración")))
        .add(sidebar::Item::link(Lc::n("Usuarios"), "/users", "people"))
        .add(sidebar::Item::link(Lc::n("Roles"), "/roles", "shield-check"));
}
```

O de forma **por página**:

```rust,no_run
use pagetop::prelude::*;
use pagetop_bootsier::theme::{BootsierRegion, sidebar};

async fn settings(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::admin(request)
        .with_child_in(
            &BootsierRegion::Sidebar,
            sidebar::Item::link(Lc::n("Ajustes"), "/settings", "gear"),
        )
        .with_child(Html::with(|_| html! { h3 { "Ajustes" } }))
        .render()
}
```

## Barra de navegación superior

La barra superior incluye por defecto los controles de pantalla completa y selector
de tema. Para añadir elementos adicionales en el lado derecho (por ejemplo, el
dropdown de usuario de `pagetop-user`), registra componentes en
`BootsierRegion::Navbar`:

```rust,no_run
use pagetop::prelude::*;
use pagetop_bootsier::theme::BootsierRegion;

InRegion::Global(&BootsierRegion::Navbar)
    .add(Html::with(|_| html! {
        li class="nav-item" {
            a class="nav-link" href="/logout" { "Cerrar sesión" }
        }
    }));
```

## Créditos

Este *crate* integra la biblioteca de estilos [Bootstrap 5.3.8](https://getbootstrap.com/) para
definir el comportamiento, la apariencia y los componentes de la interfaz. Bootstrap se distribuye
bajo licencia [MIT](https://github.com/twbs/bootstrap/blob/main/LICENSE).

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
