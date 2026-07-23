<div align="center">

<h1>PageTop HTMX</h1>

<p>Extensión para <strong>PageTop</strong> que integra <a href="https://htmx.org">HTMX</a> para enriquecer las páginas con interacciones dinámicas.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-htmx?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-htmx)
[![Crates.io](https://img.shields.io/crates/v/pagetop-htmx.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-htmx)
[![Descargas](https://img.shields.io/crates/d/pagetop-htmx.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-htmx)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-htmx#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Guía rápida

**Añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-htmx = { ... }
```

**Declara la extensión** en tu aplicación (o extensión que la requiera). Recuerda que el orden en
`dependencies()` determina la prioridad relativa frente a las otras extensiones:

```rust
use pagetop::prelude::*;

struct MyApp;

#[async_trait]
impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            // ...
            &pagetop_htmx::Htmx
            // ...
        ]
    }
}
```

A partir de ese momento, todas las páginas de la aplicación incluirán automáticamente el script de
HTMX 2. Puedes usar los atributos `hx-*` directamente en tus componentes o el código HTML generado:

```rust
use pagetop::prelude::*;

async fn homepage(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| html! {
            button hx-get="/api/hello" hx-target="#result" {
                "Say hello"
            }
            div #result {}
        }))
        .render().await
}
```

Cuando los valores se construyen en tiempo de ejecución o quieres que una extensión aplique estos
atributos sin que el componente dependa de HTMX, usa `Props` junto con las constantes de `hx` en
lugar de escribirlos como literales en `html!`:

```rust
use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

async fn homepage(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let props = Props::new(hx::GET, "/api/hello")
        .with_prop(PropsOp::set(hx::TARGET, "#result"));

    Page::new(request)
        .with_child(Html::with(move |_| html! {
            button (props) { "Say hello" }
            div #result {}
        }))
        .render().await
}
```

## Créditos

Este *crate* integra la biblioteca [HTMX 2.0.10](https://htmx.org), distribuida bajo licencia
[BSD 2-Clause](https://github.com/bigskysoftware/htmx/blob/master/LICENSE).

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
