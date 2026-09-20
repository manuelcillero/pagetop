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

La macro para generar identificadores dinámicos **`paste!`** se reexporta del *crate*
[pastey](https://crates.io/crates/pastey), una implementación avanzada y soportada del popular
`paste!` de [David Tolnay](https://crates.io/users/dtolnay).
*/

#![doc(
    html_favicon_url = "https://git.cillero.es/manuelcillero/pagetop/raw/branch/main/assets/favicon.ico"
)]

pub use indoc::{concatdoc, formatdoc, indoc};

/// Permite *pegar* tokens y generar identificadores a partir de otros.
///
/// Dentro de `paste!`, los identificadores escritos como `[< ... >]` se combinan en uno solo que
/// puede reutilizarse para referirse a items existentes o para definir nuevos (funciones,
/// estructuras, métodos, etc.).
///
/// También admite modificadores de estilo (`lower`, `upper`, `snake`, `camel`, etc.) para
/// transformar fragmentos interpolados antes de construir el nuevo identificador.
pub use pastey::paste;
// La documentación anterior se copia en `pagetop::util::paste!` porque el *crate* original no la
// define y `pagetop` no la hereda automáticamente.

/// Concatena varios fragmentos en un [`String`] reservando una sola vez la memoria del resultado.
///
/// Acepta uno o más fragmentos que implementen [`AsRef<str>`] (literales, `&str`, [`String`],
/// `Cow<str>`...) y construye el resultado con la capacidad exacta, sin pasar por el formateo. Es
/// la forma preferida de componer texto a partir de fragmentos, y suele ser más rápida que
/// `format!`.
///
/// Cada expresión se evalúa **una sola vez**, por lo que admite llamadas como `n.to_string()` o
/// cierres con efectos secundarios. Los valores que no son cadenas (números, tipos `Display`) se
/// convierten antes con `to_string()`. Si hace falta formato real (`{:02}`, `{:?}`, `{:.2}`...),
/// usar `format!`.
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
///
/// // Los fragmentos pueden ser de tipos distintos; los números se convierten antes.
/// let name = String::from("item");
/// let result_mixed = join!(&name, "-", 7.to_string());
/// assert_eq!(result_mixed, "item-7".to_string());
///
/// // Cada expresión se evalúa una sola vez.
/// let mut calls = 0;
/// let mut next = || {
///     calls += 1;
///     calls.to_string()
/// };
/// let result_once = join!("n", next());
/// assert_eq!(result_once, "n1".to_string());
/// assert_eq!(calls, 1);
/// ```
#[macro_export]
macro_rules! join {
    ($($arg:expr),+ $(,)?) => {
        [$(::core::convert::AsRef::<str>::as_ref(&$arg)),+].concat()
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

        let first = ::core::convert::AsRef::<str>::as_ref(&first_val);
        let second = ::core::convert::AsRef::<str>::as_ref(&second_val);
        let separator = if first.is_empty() || second.is_empty() {
            ""
        } else {
            ::core::convert::AsRef::<str>::as_ref(&separator_val)
        };

        [first, separator, second].concat()
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
