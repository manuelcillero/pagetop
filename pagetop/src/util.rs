//! Funciones y macros útiles.

use crate::trace;

use std::io;
use std::path::PathBuf;

// FUNCIONES ÚTILES ********************************************************************************

pub enum TypeInfo {
    FullName,
    ShortName,
    NameFrom(isize),
    NameTo(isize),
    PartialName(isize, isize),
}

impl TypeInfo {
    pub fn of<T: ?Sized>(&self) -> &'static str {
        let type_name = std::any::type_name::<T>();
        match self {
            TypeInfo::FullName => type_name,
            TypeInfo::ShortName => Self::partial(type_name, -1, None),
            TypeInfo::NameFrom(start) => Self::partial(type_name, *start, None),
            TypeInfo::NameTo(end) => Self::partial(type_name, 0, Some(*end)),
            TypeInfo::PartialName(start, end) => Self::partial(type_name, *start, Some(*end)),
        }
    }

    fn partial(type_name: &'static str, start: isize, end: Option<isize>) -> &'static str {
        let maxlen = type_name.len();
        let mut segments = Vec::new();
        let mut segment_start = 0; // Posición de inicial del segmento actual.
        let mut angle_brackets = 0; // Contador para seguimiento de '<' y '>'.
        let mut previous_char = '\0'; // Se inicializa a carácter nulo, no hay aún carácter previo.

        for (idx, c) in type_name.char_indices() {
            match c {
                ':' if angle_brackets == 0 => {
                    if previous_char == ':' {
                        if segment_start < idx - 1 {
                            segments.push((segment_start, idx - 1)); // No incluye último '::'.
                        }
                        segment_start = idx + 1; // El siguiente segmento comienza después de '::'.
                    }
                }
                '<' => angle_brackets += 1,
                '>' => angle_brackets -= 1,
                _ => {}
            }
            previous_char = c;
        }

        // Incluye el último segmento si lo hubiese.
        if segment_start < maxlen {
            segments.push((segment_start, maxlen));
        }

        // Calcula la posición inicial.
        let start_pos = segments
            .get(if start >= 0 {
                start as usize
            } else {
                segments.len() - start.unsigned_abs()
            })
            .map_or(0, |&(s, _)| s);

        // Calcula la posición final.
        let end_pos = segments
            .get(if let Some(end) = end {
                if end >= 0 {
                    end as usize
                } else {
                    segments.len() - end.unsigned_abs()
                }
            } else {
                segments.len() - 1
            })
            .map_or(maxlen, |&(_, e)| e);

        // Devuelve la cadena parcial basada en las posiciones calculadas.
        &type_name[start_pos..end_pos]
    }
}

/// Calcula el directorio absoluto dado un directorio raíz y una ruta relativa.
///
/// # Argumentos
///
/// * `root_path` - Contiene el directorio raíz.
/// * `relative_path` - Contiene la ruta relativa.
///
/// # Devuelve
///
/// * `Ok` - Si la operación es correcta devuelve el directorio absoluto como un `String`.
/// * `Err` - Si ocurre un error de E/S, devuelve un `io::Error`.
///
/// # Errores
///
/// Esta función devolverá un error si:
/// - El directorio raíz o la ruta relativa son inválidos.
/// - Hay un problema con las operaciones sobre el sistema de archivos, como leer el directorio.
///
/// # Ejemplos
///
/// ```rust#ignore
/// let root = "/home/user";
/// let relative = "documents";
/// let abs_dir = absolute_dir(root, relative).unwrap();
/// println!("{}", abs_dir);
/// ```
pub fn absolute_dir(
    root_path: impl Into<String>,
    relative_path: impl Into<String>,
) -> Result<String, io::Error> {
    let root_path = PathBuf::from(root_path.into());
    let full_path = root_path.join(relative_path.into());
    let absolute_dir = full_path.to_string_lossy().into();

    if !full_path.is_absolute() {
        let message = format!("Path \"{absolute_dir}\" is not absolute");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
    }

    if !full_path.exists() {
        let message = format!("Path \"{absolute_dir}\" does not exist");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::NotFound, message));
    }

    if !full_path.is_dir() {
        let message = format!("Path \"{absolute_dir}\" is not a directory");
        trace::warn!(message);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, message));
    }

    Ok(absolute_dir)
}

// MACROS ÚTILES ***********************************************************************************

#[doc(hidden)]
pub use paste::paste;

#[doc(hidden)]
pub use concat_string::concat_string;

#[macro_export]
/// Macro para construir una colección de pares clave-valor.
///
/// ```rust#ignore
/// let args = hm![
///     "userName" => "Roberto",
///     "photoCount" => 3,
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

/// Concatena varios fragmentos de cadenas (*string slices*) en una cadena *String*.
///
/// Exporta la macro [`concat_string!`](https://docs.rs/concat-string), que permite concatenar de
/// forma eficiente fragmentos de cadenas en una cadena *String*. Acepta cualquier número de
/// argumentos que implementen `AsRef<str>` y crea una cadena `String` con el tamaño adecuado, sin
/// requerir cadenas de formato que puedan sobrecargar el rendimiento.
///
/// # Ejemplo
///
/// ```rust#ignore
/// // Concatena todos los fragmentos directamente.
/// let result = join_string!("Hello", " ", "World");
/// assert_eq!(result, "Hello World".to_string());
///
/// // También funciona con valores vacíos.
/// let result_with_empty = join_string!("Hello", "", "World");
/// assert_eq!(result_with_empty, "HelloWorld".to_string());
///
/// // Un único fragmento devuelve el mismo valor.
/// let single_result = join_string!("Hello");
/// assert_eq!(single_result, "Hello".to_string());
/// ```
#[macro_export]
macro_rules! join_string {
    ($($arg:tt)*) => {
        $crate::util::concat_string!($($arg)*)
    };
}

/// Concatena varios fragmentos de cadenas (*string slices*) en una cadena *String* utilizando
/// opcionalmente un separador.
///
/// Crea una cadena que contiene los fragmentos no vacíos concatenados. La macro puede utilizar un
/// separador explícito o concatenar directamente los fragmentos sin un separador. Acepta cualquier
/// número de argumentos que implementen `AsRef<str>`.
///
/// Si todos los fragmentos son cadenas vacías, devuelve `None`.
///
/// # Ejemplo
///
/// ```rust#ignore
/// // Concatena los fragmentos no vacíos con un espacio como separador.
/// let result_with_separator = option_string!(["Hello", "", "World"]; " ");
/// assert_eq!(result_with_separator, Some("Hello World".to_string()));
///
/// // Concatena los fragmentos no vacíos sin un separador.
/// let result_without_separator = option_string!(["Hello", "", "World"]);
/// assert_eq!(result_without_separator, Some("HelloWorld".to_string()));
///
/// // Devuelve `None` si todos los fragmentos están vacíos.
/// let result_empty = option_string!(["", "", ""]);
/// assert_eq!(result_empty, None);
/// ```
#[macro_export]
macro_rules! option_string {
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

/// Concatena dos fragmentos de cadenas (*string slices*) en una cadena *String* con un separador.
///
/// Concatena los dos fragmentos que implementen `AsRef<str>` usando el separador proporcionado,
/// pero devuelve directamente el primer fragmento si el segundo está vacío, o el segundo fragmento
/// si el primero está vacío.
///
/// # Ejemplo
///
/// ```rust#ignore
/// let first = "Hello";
/// let separator = "-";
/// let second = "World";
///
/// // Concatena los dos fragmentos cuando ambos no están vacíos.
/// let result = trio_string!(first, separator, second);
/// assert_eq!(result, "Hello-World".to_string());
///
/// // Si el primer fragmento está vacío, devuelve el segundo.
/// let result_empty_first = trio_string!("", separator, second);
/// assert_eq!(result_empty_first, "World".to_string());
///
/// // Si el segundo fragmento está vacío, devuelve el primero.
/// let result_empty_second = trio_string!(first, separator, "");
/// assert_eq!(result_empty_second, "Hello".to_string());
///
/// // Si ambos fragmentos están vacíos, devuelve una cadena vacía.
/// let result_both_empty = trio_string!("", separator, "");
/// assert_eq!(result_both_empty, "".to_string());
/// ```
#[macro_export]
macro_rules! trio_string {
    ($first:expr, $separator:expr, $second:expr) => {{
        if $first.is_empty() {
            $second.to_string()
        } else if $second.is_empty() {
            $first.to_string()
        } else {
            $crate::util::concat_string!($first, $separator, $second)
        }
    }};
}
