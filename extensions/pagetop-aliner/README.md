<div align="center">

<h1>PageTop Aliner</h1>

<p>Tema de <strong>PageTop</strong> que muestra esquemáticamente la composición de las páginas HTML.</p>

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


# 🚧 Advertencia

**PageTop** es un proyecto personal para aprender [Rust](https://www.rust-lang.org/es) y conocer su
ecosistema. Su API está sujeta a cambios frecuentes. No se recomienda su uso en producción, al menos
hasta que se libere la versión **1.0.0**.


# 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
