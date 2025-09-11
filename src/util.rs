//! Funciones y macros útiles.

use crate::trace;

use std::env;
use std::io;
use std::path::{Path, PathBuf};

// FUNCIONES ÚTILES ********************************************************************************

/// Resuelve y valida la ruta de un directorio existente, devolviendo una ruta absoluta.
///
/// - Si la ruta es relativa, se resuelve respecto al directorio del proyecto según la variable de
///   entorno `CARGO_MANIFEST_DIR` (si existe) o, en su defecto, respecto al directorio actual de
///   trabajo.
/// - Normaliza y valida la ruta final (resuelve `.`/`..` y enlaces simbólicos).
/// - Devuelve error si la ruta no existe o no es un directorio.
///
/// # Ejemplos
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// // Ruta relativa, se resuelve respecto a CARGO_MANIFEST_DIR o al directorio actual (`cwd`).
/// println!("{:#?}", util::resolve_absolute_dir("documents"));
///
/// // Ruta absoluta, se normaliza y valida tal cual.
/// println!("{:#?}", util::resolve_absolute_dir("/var/www"));
/// ```
pub fn resolve_absolute_dir<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Directorio base CARGO_MANIFEST_DIR si está disponible; o current_dir() en su defecto.
        env::var_os("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .or_else(|| env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."))
            .join(path)
    };

    // Resuelve `.`/`..`, enlaces simbólicos y obtiene la ruta absoluta en un único paso.
    let absolute_dir = candidate.canonicalize()?;

    // Asegura que realmente es un directorio existente.
    if absolute_dir.is_dir() {
        Ok(absolute_dir)
    } else {
        Err({
            let msg = format!("Path \"{}\" is not a directory", absolute_dir.display());
            trace::warn!(msg);
            io::Error::new(io::ErrorKind::InvalidInput, msg)
        })
    }
}

/// **Obsoleto desde la versión 0.3.0**: usar [`resolve_absolute_dir()`] en su lugar.
#[deprecated(since = "0.3.0", note = "Use `resolve_absolute_dir()` instead")]
pub fn absolute_dir<P, Q>(root_path: P, relative_path: Q) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    resolve_absolute_dir(root_path.as_ref().join(relative_path.as_ref()))
}

// MACROS ÚTILES ***********************************************************************************

#[doc(hidden)]
pub use paste::paste;

#[doc(hidden)]
pub use concat_string::concat_string;

#[macro_export]
/// Macro para construir una colección de pares clave-valor.
///
/// ```rust
/// use pagetop::hm;
/// use std::collections::HashMap;
///
/// let args:HashMap<&str, String> = hm![
///     "userName"   => "Roberto",
///     "photoCount" => "3",
///     "userGender" => "male",
/// ];
/// ```
macro_rules! hm {
    ( $($key:expr => $value:expr),* $(,)? ) => {{
        let mut a = std::collections::HashMap::new();
        $(
            a.insert($key.into(), $value.into());
        )*
        a
    }};
}

/// Concatena eficientemente varios fragmentos en un [`String`].
///
/// Esta macro exporta [`concat_string!`](https://docs.rs/concat-string). Acepta cualquier número de
/// fragmentos que implementen [`AsRef<str>`] y construye un [`String`] con el tamaño óptimo, de
/// forma eficiente y evitando el uso de cadenas de formato que penalicen el rendimiento.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
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
    ($($arg:tt)*) => {
        $crate::util::concat_string!($($arg)*)
    };
}

/// Concatena los fragmentos no vacíos en un [`Option<String>`] con un separador opcional.
///
/// Esta macro acepta cualquier número de fragmentos que implementen [`AsRef<str>`] para concatenar
/// todos los fragmentos no vacíos usando opcionalmente un separador.
///
/// Si todos los fragmentos están vacíos, devuelve [`None`].
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Concatena los fragmentos no vacíos con un espacio como separador.
/// let result_with_separator = join_opt!(["Hello", "", "World"]; " ");
/// assert_eq!(result_with_separator, Some("Hello World".to_string()));
///
/// // Concatena los fragmentos no vacíos sin un separador.
/// let result_without_separator = join_opt!(["Hello", "", "World"]);
/// assert_eq!(result_without_separator, Some("HelloWorld".to_string()));
///
/// // Devuelve `None` si todos los fragmentos están vacíos.
/// let result_empty = join_opt!(["", "", ""]);
/// assert_eq!(result_empty, None);
/// ```
#[macro_export]
macro_rules! join_opt {
    ([$($arg:expr),* $(,)?]) => {{
        let s = $crate::util::concat_string!($($arg),*);
        (!s.is_empty()).then_some(s)
    }};
    ([$($arg:expr),* $(,)?]; $separator:expr) => {{
        let s = [$($arg),*]
            .iter()
            .filter(|&item| !item.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join($separator);
        (!s.is_empty()).then_some(s)
    }};
}

/// Concatena dos fragmentos en un [`String`] usando un separador.
///
/// Une los dos fragmentos, que deben implementar [`AsRef<str>`], usando el separador proporcionado.
/// Si uno de ellos está vacío, devuelve directamente el otro; y si ambos están vacíos devuelve un
/// [`String`] vacío.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
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
        if $first.is_empty() {
            String::from($second)
        } else if $second.is_empty() {
            String::from($first)
        } else {
            $crate::util::concat_string!($first, $separator, $second)
        }
    }};
}

/// Concatena varios fragmentos en un [`Option<String>`] si ninguno está vacío.
///
/// Si alguno de los fragmentos, que deben implementar [`AsRef<str>`], está vacío, devuelve
/// [`None`]. Opcionalmente se puede indicar un separador entre los fragmentos concatenados.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Concatena los fragmentos.
/// let result = join_strict!(["Hello", "World"]);
/// assert_eq!(result, Some("HelloWorld".to_string()));
///
/// // Concatena los fragmentos con un separador.
/// let result_with_separator = join_strict!(["Hello", "World"]; " ");
/// assert_eq!(result_with_separator, Some("Hello World".to_string()));
///
/// // Devuelve `None` si alguno de los fragmentos está vacío.
/// let result_with_empty = join_strict!(["Hello", "", "World"]);
/// assert_eq!(result_with_empty, None);
/// ```
#[macro_export]
macro_rules! join_strict {
    ([$($arg:expr),* $(,)?]) => {{
        let fragments = [$($arg),*];
        if fragments.iter().any(|&item| item.is_empty()) {
            None
        } else {
            Some(fragments.concat())
        }
    }};
    ([$($arg:expr),* $(,)?]; $separator:expr) => {{
        let fragments = [$($arg),*];
        if fragments.iter().any(|&item| item.is_empty()) {
            None
        } else {
            Some(fragments.join($separator))
        }
    }};
}
