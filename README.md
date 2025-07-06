<div align="center">

<h1>PageTop</h1>

<p>Un entorno para el desarrollo de soluciones web modulares, extensibles y configurables.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)

</div>

`PageTop` reivindica la esencia de la web clásica usando [Rust](https://www.rust-lang.org/es) para
la creación de soluciones web SSR (*renderizadas en el servidor*) basadas en HTML, CSS y JavaScript.


# ⚡️ Guía rápida

La aplicación más sencilla de `PageTop` se ve así:

```rust
use pagetop::prelude::*;

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::new().run()?.await
}
```


# 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.


# ✨ Contribuir

Cualquier contribución para añadir al proyecto se considerará automáticamente bajo la doble licencia
indicada arriba (MIT o Apache v2.0), sin términos o condiciones adicionales, tal y como permite la
licencia *Apache v2.0*.
