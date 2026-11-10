/*!
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
HTMX 2, y las que tengan algún listado ([`ListView`]), unos pocos estilos que lo atenúan mientras
se actualiza. Puedes usar los atributos `hx-*` directamente en tus componentes o el código HTML
generado:

```rust
use pagetop::prelude::*;

async fn homepage(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| html! {
            button hx-get="/api/hello" hx-target="#result" {
                "Say hello"
            }
            div id="result" {}
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
        .with_child(Html::with(move |cx| html! {
            button (props.unpack(cx)) { "Say hello" }
            div id="result" {}
        }))
        .render().await
}
```
*/

use pagetop::prelude::*;

include_locales!(LOCALES_HTMX);

pub mod hx;
pub mod hx_filter;
pub mod hx_pager;
pub mod hx_table;
pub mod request;
pub mod response;

/// Prelude de `pagetop-htmx`.
pub mod prelude {
    pub use crate::hx;
    pub use crate::hx_filter;
    pub use crate::hx_pager;
    pub use crate::hx_table;
    pub use crate::request::HtmxRequestExt;
    pub use crate::response::HtmxResponse;
}

/// Integra HTMX 2 en cualquier aplicación PageTop.
///
/// Poner esta extensión en [`dependencies()`](pagetop::core::extension::Extension::dependencies)
/// hace que todas las páginas de la aplicación incluyan automáticamente el script de HTMX mediante
/// un atributo [`defer`](pagetop::html::JavaScript::defer), y las que tienen algún listado
/// ([`ListView`]), los estilos con los que se atenúa mientras se actualiza. No es necesaria ninguna
/// configuración adicional.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// struct MyApp;
///
/// #[async_trait]
/// impl Extension for MyApp {
///     fn dependencies(&self) -> Vec<ExtensionRef> {
///         vec![
///             // ...
///             &pagetop_htmx::Htmx
///             // ...
///         ]
///     }
/// }
/// ```
pub struct Htmx;

#[async_trait]
impl Extension for Htmx {
    fn name(&self) -> Lc {
        Lc::t("extension_name", &LOCALES_HTMX)
    }

    fn description(&self) -> Lc {
        Lc::t("extension_description", &LOCALES_HTMX)
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::page::BeforeRenderBody::new(add_htmx_script),
            action::component::BeforeRender::<ListView>::new(add_list_loading_styles),
        ]
    }

    fn configure_static_files(&self, files: StaticFiles) -> StaticFiles {
        serve_static_files!(files, [htmx] => "/htmx");
        files
    }
}

fn add_htmx_script(page: &mut Page) {
    page.alter_assets(JavaScript::defer("/htmx/js/htmx.min.js").with_version("2.0.10"));
}

// Mientras los helpers de listado (filtros, paginador, cabeceras ordenables) esperan una respuesta,
// HTMX pone la clase `htmx-request` en el listado: se atenúa para que se vea que está cargando. El
// retardo evita el parpadeo cuando la respuesta es rápida. Sólo en las páginas con algún listado.
fn add_list_loading_styles(_list: &mut ListView, cx: &mut Context) {
    cx.alter_assets(AssetsOp::add_responsive_styles(
        None,
        "list-view",
        [("transition", "opacity 0.2s ease-in-out 0.15s")],
    ));
    cx.alter_assets(AssetsOp::add_responsive_styles(
        None,
        "list-view htmx-request",
        [("opacity", "0.5")],
    ));
}
