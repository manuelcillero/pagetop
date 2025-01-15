# Prepara tu entorno

PageTop está escrito en Rust. Antes de empezar a crear tu aplicación web, es importante dedicar un tiempo a preparar tu entorno de desarrollo con Rust.

## Instalación de Rust

PageTop depende en gran medida de las mejoras que se aplican en el lenguaje y el compilador Rust. Procura tener instalada "*la última versión estable*" para admitir la Versión Mínima de Rust Soportada (MSRV) por PageTop.

Puedes instalar Rust siguiendo la [Guía de Inicio Rápido de Rust](https://www.rust-lang.org/learn/get-started).

Una vez completada la instalación, tendrás disponibles en tu sistema el compilador `rustc` y `cargo` para la construcción y gestión de *crates* de Rust.

## Recursos para aprender Rust

El objetivo de esta guía es aprender a programar con PageTop rápidamente, por lo que no te va a servir como material de aprendizaje de Rust. Si deseas saber más sobre el [lenguaje de programación Rust](https://www.rust-lang.org), consulta los siguientes recursos:

  * **[El Libro de Rust](https://doc.rust-lang.org/book/)**: el mejor lugar para aprender Rust desde cero.
  * **[Rust con Ejemplos](https://doc.rust-lang.org/rust-by-example/)**: aprende Rust programando ejemplos de todo tipo.
  * **[Rustlings](https://github.com/rust-lang/rustlings)**: una serie de ejercicios divertidos e interactivos para conocer Rust.

## Editor de código / IDE

Puedes usar tu editor de código preferido, pero se recomienda uno que permita instalar la extensión de [rust-analyzer](https://github.com/rust-lang/rust-analyzer). Aunque aún está en desarrollo, proporciona autocompletado y una inteligencia de código avanzada. [Visual Studio Code](https://code.visualstudio.com/) tiene una [extensión de rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) oficialmente soportada.


# Tu primer proyecto PageTop

¡Ha llegado el momento de programar con PageTop! Inicialmente PageTop es sólo una dependencia más en tu proyecto. Puedes añadirlo a un proyecto ya existente o crear uno nuevo. Para ser completos, asumiremos que estás empezando desde cero.

## Crea un nuevo proyecto de ejecutable Rust

Primero, navega a una carpeta donde quieras crear tu nuevo proyecto. Luego, ejecuta el siguiente comando para crear una nueva carpeta que contenga nuestro proyecto de ejecutable Rust:

```bash
cargo new my_pagetop_app
cd my_pagetop_app
```

Ahora ejecuta `cargo run` para compilar y ejecutar tu proyecto. Deberías ver el texto "Hello, world!" en tu terminal. Abre la carpeta "my_pagetop_app" en tu editor de código preferido y tómate un tiempo para revisar los archivos.

`main.rs` es el punto de entrada de tu programa:

```rust
fn main() {
    println!("Hello, world!");
}
```

`Cargo.toml` es tu "archivo de proyecto". Contiene metadatos sobre tu proyecto, como su nombre, dependencias y configuración para compilarlo.

```toml
[package]
name = "my_pagetop_app"
version = "0.1.0"
edition = "2021"

[dependencies]
```

## Añade PageTop como dependencia

PageTop está disponible como [biblioteca en crates.io](https://crates.io/crates/pagetop), el repositorio oficial de *crates* Rust.

La forma más fácil de incorporarlo en tu proyecto es usar `cargo add`:

```bash
cargo add pagetop
```

O puedes añadirlo manualmente en el archivo `Cargo.toml` del proyecto escribiendo:

```toml
[package]
name = "my_pagetop_app"
version = "0.1.0"
edition = "2021" # debe ser 2021, o necesitarás configurar "resolver=2"

[dependencies]
pagetop = "0.0.X" # siendo X la última versión de desarrollo
```

Asegúrate de que se añade la última versión disponible de PageTop:

[![Crates.io](https://img.shields.io/crates/v/pagetop.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop)

## Construye PageTop

Ahora ejecuta `cargo run` nuevamente. Las dependencias de PageTop deberían comenzar a compilarse. Tomará algo de tiempo ya que es la primera compilación de tu proyecto con PageTop. Esto sólo ocurrirá la primera vez. ¡Cada compilación después de esta será más rápida!

Ahora que tenemos nuestro proyecto PageTop preparado, ¡estamos listos para programar nuestra primera aplicación PageTop!