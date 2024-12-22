<div align="center">

<h1>PageTop HighlightJS</h1>

<p>Integra HighlightJS para mostrar fragmentos de código con resaltado de sintaxis con <strong>PageTop</strong>.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
[![Doc API](https://img.shields.io/docsrs/pagetop-hljs?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-hljs)
[![Crates.io](https://img.shields.io/crates/v/pagetop-hljs.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-hljs)
[![Descargas](https://img.shields.io/crates/d/pagetop-hljs.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-hljs)

</div>

## Descripción general

  * Utiliza la versátil librería JavaScript [highlight.js](https://highlightjs.org/).
  * Soporta **más de 90** lenguajes de programación.
  * Ofrece **más de 95** temas disponibles para elegir.
  * Proporciona un componente para mostrar fragmentos de código fácilmente.
  * Permite resaltar bloques de código multilínea.
  * Detecta prefijos de clase `language-` y `lang-`.
  * Permite personalizar la inicialización de *highlight.js*.
  * Carga inteligente de recursos CSS y JS.

## Uso

Añade `pagetop-hljs` a tu archivo `Cargo.toml`:

```rust
[dependencies]
pagetop-hljs = "<Version>"
```

Incluye `pagetop_hljs::HighlightJS` en las dependencias del paquete o aplicación que lo requiera:

```rust#ignore
use pagetop::prelude::*;

impl PackageTrait for MyPackage {
    // ...
    fn dependencies(&self) -> Vec<PackageRef> {
        vec![
            // ...
            &pagetop_hljs::HighlightJS,
            // ...
        ]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        scfg.route("/snippet", service::web::get().to(hljs_sample));
    }
    // ...
}
```

Y finalmente añade tus fragmentos de código con resaltado de sintaxis en páginas web:

```rust#ignore
use pagetop_hljs::prelude::*;

async fn hljs_sample(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_component(Snippet::with(
            HljsLang::Rust,
            r###"
// This is the main function.
fn main() {
    // Print text to the console.
    println!("Hello World!");
}
            "###,
        ))
        .render()
}
```


# 📌 Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.


# 🚧 Advertencia

`PageTop` es un proyecto personal que hago por diversión para aprender cosas nuevas. Está en
desarrollo activo, su API es inestable y está sujeta a cambios frecuentes. No recomiendo su uso en
producción, al menos hasta liberar la versión **1.0.0**.


# 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
