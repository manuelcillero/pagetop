/*!
<div align="center">

<h1>PageTop Minimal</h1>

<p>Reúne un conjunto mínimo de macros para mejorar el formato y la eficiencia de operaciones básicas en <strong>PageTop</strong>.</p>

[![Doc API](https://img.shields.io/docsrs/pagetop-minimal?label=Doc%20API&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-minimal)
[![Crates.io](https://img.shields.io/crates/v/pagetop-minimal.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-minimal)
[![Descargas](https://img.shields.io/crates/d/pagetop-minimal.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-minimal)
[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](https://git.cillero.es/manuelcillero/pagetop/src/branch/main/helpers/pagetop-minimal#licencia)

</div>

## Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.

## Descripción general

Este *crate* proporciona un conjunto básico de macros que se integran en las utilidades de PageTop
para optimizar operaciones habituales relacionadas con la composición estructurada de texto, la
concatenación de cadenas y el uso rápido de colecciones clave-valor.

## Créditos

Las macros para texto multilínea **`indoc!`**, **`formatdoc!`** y **`concatdoc!`** se reexportan del
*crate* [indoc](https://crates.io/crates/indoc) de [David Tolnay](https://crates.io/users/dtolnay).

Las macros para la concatenación de cadenas **`join!`** y **`join_pair!`** se apoyan internamente en
el **crate** [concat-string](https://crates.io/crates/concat_string), desarrollado por
[FaultyRAM](https://crates.io/users/FaultyRAM), para evitar el uso del formato de cadenas cuando la
eficiencia pueda ser relevante.
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/static/favicon.ico"
)]

#[doc(hidden)]
pub use concat_string::concat_string;

pub use indoc::{concatdoc, formatdoc, indoc};

/// Concatena eficientemente varios fragmentos en un [`String`].
///
/// Esta macro exporta [`concat_string!`](https://docs.rs/concat-string). Acepta cualquier número de
/// fragmentos que implementen [`AsRef<str>`] y construye un [`String`] con el tamaño óptimo, de
/// forma eficiente y evitando el uso de cadenas de formato que penalicen el rendimiento.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop_minimal::join;
/// // Concatena todos los fragmentos directamente.
/// let result = join!("Hello", " ", "World");
/// assert_eq!(result, "Hello World".to_string());
///
/// // También funciona con valores vacíos.
/// let result_with_empty = join!("Hello", "", "World");
/// assert_eq!(result_with_empty, "HelloWorld".to_string());
///
/// // Un único fragmento devuelve el mismo valor.
/// let single_result = join!("Hello");
/// assert_eq!(single_result, "Hello".to_string());
/// ```
#[macro_export]
macro_rules! join {
    ($($arg:expr),+) => {
        $crate::concat_string!($($arg),+)
    };
}

/// Concatena dos fragmentos en un [`String`] usando un separador inteligente.
///
/// Une los dos fragmentos, que deben implementar [`AsRef<str>`], usando el separador proporcionado.
/// Si uno de ellos está vacío, devuelve directamente el otro; y si ambos están vacíos devuelve un
/// [`String`] vacío.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop_minimal::join_pair;
/// let first = "Hello";
/// let separator = "-";
/// let second = "World";
///
/// // Concatena los dos fragmentos cuando ambos no están vacíos.
/// let result = join_pair!(first, separator, second);
/// assert_eq!(result, "Hello-World".to_string());
///
/// // Si el primer fragmento está vacío, devuelve el segundo.
/// let result_empty_first = join_pair!("", separator, second);
/// assert_eq!(result_empty_first, "World".to_string());
///
/// // Si el segundo fragmento está vacío, devuelve el primero.
/// let result_empty_second = join_pair!(first, separator, "");
/// assert_eq!(result_empty_second, "Hello".to_string());
///
/// // Si ambos fragmentos están vacíos, devuelve una cadena vacía.
/// let result_both_empty = join_pair!("", separator, "");
/// assert_eq!(result_both_empty, "".to_string());
/// ```
#[macro_export]
macro_rules! join_pair {
    ($first:expr, $separator:expr, $second:expr) => {{
        let first_val = $first;
        let second_val = $second;
        let separator_val = $separator;

        let first = AsRef::<str>::as_ref(&first_val);
        let second = AsRef::<str>::as_ref(&second_val);
        let separator = if first.is_empty() || second.is_empty() {
            ""
        } else {
            AsRef::<str>::as_ref(&separator_val)
        };

        $crate::concat_string!(first, separator, second)
    }};
}

/// Macro para construir una colección de pares clave-valor.
///
/// ```rust
/// # use pagetop_minimal::kv;
/// # use std::collections::HashMap;
/// let args:HashMap<&str, String> = kv![
///     "userName"   => "Roberto",
///     "photoCount" => "3",
///     "userGender" => "male",
/// ];
/// ```
#[macro_export]
macro_rules! kv {
    ( $($key:expr => $value:expr),* $(,)? ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert($key.into(), $value.into());
        )*
        a
    }};
}
