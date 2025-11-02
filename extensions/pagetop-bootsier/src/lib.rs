/*!
<div align="center">

<h1>PageTop Bootsier</h1>

<p>Tema de <strong>PageTop</strong> basado en Bootstrap para aplicar su catálogo de estilos y componentes flexibles.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-bootsier?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-bootsier)
[![Crates.io](https://img.shields.io/crates/v/pagetop-bootsier.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-bootsier)
[![Descargas](https://img.shields.io/crates/d/pagetop-bootsier.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-bootsier)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-bootsier#licencia)

<br>
</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.


# ⚡️ Guía rápida

Igual que con otras extensiones, **añade la dependencia** a tu `Cargo.toml`:

```toml
[dependencies]
pagetop-bootsier = "..."
```

**Declara la extensión** en tu aplicación (o extensión que la requiera). Recuerda que el orden en
`dependencies()` determina la prioridad relativa frente a las otras extensiones:

```rust,no_run
use pagetop::prelude::*;

struct MyApp;

impl Extension for MyApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            // ...
            &pagetop_bootsier::Bootsier,
            // ...
        ]
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&MyApp).run()?.await
}
```

Y **selecciona el tema en la configuración** de la aplicación:

```toml
[app]
theme = "Bootsier"
```

…o **fuerza el tema por código** en una página concreta:

```rust,no_run
use pagetop::prelude::*;

async fn homepage(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_theme("Bootsier")
        .add_child(
            Block::new()
                .with_title(L10n::l("sample_title"))
                .add_child(Html::with(|cx| html! {
                    p { (L10n::l("sample_content").using(cx)) }
                })),
        )
        .render()
}
```
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/favicon.ico"
)]

use pagetop::prelude::*;

include_locales!(LOCALES_BOOTSIER);

// Versión de la librería Bootstrap.
const BOOTSTRAP_VERSION: &str = "5.3.8";

pub mod config;

pub mod theme;

/// *Prelude* del tema.
pub mod prelude {
    pub use crate::config::*;
    pub use crate::theme::aux::*;
    pub use crate::theme::*;
}

/// El tema usa las mismas regiones predefinidas por [`ThemeRegion`].
pub type BootsierRegion = ThemeRegion;

/// Implementa el tema.
pub struct Bootsier;

impl Extension for Bootsier {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, [bootsier_bs] => "/bootsier/bs");
        static_files_service!(scfg, [bootsier_js] => "/bootsier/js");
    }
}

impl Theme for Bootsier {
    fn after_render_page_body(&self, page: &mut Page) {
        page.alter_assets(ContextOp::AddStyleSheet(
            StyleSheet::from("/bootsier/bs/bootstrap.min.css")
                .with_version(BOOTSTRAP_VERSION)
                .with_weight(-90),
        ))
        .alter_assets(ContextOp::AddJavaScript(
            JavaScript::defer("/bootsier/js/bootstrap.bundle.min.js")
                .with_version(BOOTSTRAP_VERSION)
                .with_weight(-90),
        ));
    }
}
