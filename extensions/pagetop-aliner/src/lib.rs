/*!
<div align="center">

<h1>PageTop Aliner</h1>

<p>Tema para <strong>PageTop</strong> que muestra esquemáticamente la composición de las páginas HTML.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-aliner?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-aliner)
[![Crates.io](https://img.shields.io/crates/v/pagetop-aliner.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-aliner)
[![Descargas](https://img.shields.io/crates/d/pagetop-aliner.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-aliner)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/extensions/pagetop-aliner#licencia)

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
pagetop-aliner = "..."
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
            &pagetop_aliner::Aliner,
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
theme = "Aliner"
```

…o **fuerza el tema por código** en una página concreta:

```rust,no_run
use pagetop::prelude::*;

async fn homepage(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_theme("Aliner")
        .add_component(
            Block::new()
                .with_title(L10n::l("sample_title"))
                .add_component(Html::with(|cx| html! {
                    p { (L10n::l("sample_content").using(cx)) }
                })),
        )
        .render()
}
```
*/

use pagetop::prelude::*;

/// El tema usa las mismas regiones predefinidas por [`ThemeRegion`].
pub type AlinerRegion = ThemeRegion;

/// Implementa el tema para usar en pruebas que muestran el esquema de páginas HTML.
///
/// Tema mínimo ideal para **pruebas y demos** que renderiza el **esqueleto HTML** con las mismas
/// regiones básicas definidas por [`ThemeRegion`]. No pretende ser un tema para producción, está
/// pensado para:
///
/// - Verificar integración de componentes y composiciones (*layouts*) sin estilos complejos.
/// - Realizar pruebas de renderizado rápido con salida estable y predecible.
/// - Preparar ejemplos y documentación, sin dependencias visuales (CSS/JS) innecesarias.
pub struct Aliner;

impl Extension for Aliner {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, [aliner] => "/aliner");
    }
}

impl Theme for Aliner {
    fn after_render_page_body(&self, page: &mut Page) {
        page.alter_param("include_basic_assets", true)
            .alter_assets(ContextOp::AddStyleSheet(
                StyleSheet::from("/aliner/css/styles.css")
                    .with_version(env!("CARGO_PKG_VERSION"))
                    .with_weight(-90),
            ));
    }
}
