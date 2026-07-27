//! Macros y funciones útiles.

use crate::trace;

use std::borrow::Cow;
use std::env;
use std::io;
use std::path::{Path, PathBuf};

// **< MACROS INTEGRADAS >**************************************************************************

pub use pagetop_minimal::{concatdoc, formatdoc, indoc, join, join_pair, kv};

/// Permite *pegar* tokens y generar identificadores a partir de otros.
///
/// Dentro de `paste!`, los identificadores escritos como `[< ... >]` se combinan en uno solo que
/// puede reutilizarse para referirse a items existentes o para definir nuevos (funciones,
/// estructuras, métodos, etc.).
///
/// También admite modificadores de estilo (`lower`, `upper`, `snake`, `camel`, etc.) para
/// transformar fragmentos interpolados antes de construir el nuevo identificador.
pub use pagetop_minimal::paste;
// La documentación anterior está copiada de `pagetop_minimal::paste!` porque el *crate* original
// no la define y la de `pagetop_minimal` no se hereda automáticamente.

// **< FUNCIONES ÚTILES >***************************************************************************

/// Código requerido por las macros [`pagetop::main`](crate::main) y [`pagetop::test`](crate::test).
///
/// Equivale a `Builder::new_multi_thread().enable_all().build()`, el mismo runtime que construye
/// `#[tokio::main]`/`#[tokio::test(flavor = "multi_thread")]` sin argumentos. Las aplicaciones o
/// extensiones que usen estas macros no necesitan declarar `tokio` como dependencia directa en su
/// `Cargo.toml`.
///
/// Esta función no forma parte de la API pública; es `pub` para ser llamada por el código generado
/// por las macros.
#[doc(hidden)]
pub fn build_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("Failed to build the Tokio runtime")
}

/// Errores posibles al normalizar una cadena ASCII con [`normalize_ascii()`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizeAsciiError {
    /// La entrada está vacía (`""`).
    IsEmpty,
    /// La entrada quedó vacía tras recortar separadores ASCII al inicio/fin.
    EmptyAfterTrimming,
    /// La entrada contiene al menos un byte no ASCII (>= 0x80).
    NonAscii,
}

/// Recorta espacios y convierte una cadena vacía en `None`.
///
/// Pensada para campos de formulario opcionales, de tal manera que si una cadena queda vacía tras
/// recortar, entonces se trata como "no proporcionado" en lugar de como un valor válido.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::util;
/// assert_eq!(util::non_blank("  hello  "), Some("hello"));
/// assert_eq!(util::non_blank("   "), None);
/// assert_eq!(util::non_blank(""), None);
/// ```
#[inline]
pub fn non_blank(s: &str) -> Option<&str> {
    let s = s.trim();
    if s.is_empty() { None } else { Some(s) }
}

/// Normaliza una cadena ASCII con uno o varios tokens separados.
///
/// Los *separadores* son caracteres `is_ascii_whitespace()` como `' '`, `'\t'`, `'\n'` o `'\r'`.
///
/// Reglas:
///
/// - Devuelve `Err(NormalizeAsciiError::IsEmpty)` si la entrada es `""`.
/// - Devuelve `Err(NormalizeAsciiError::NonAscii)` si contiene algún byte no ASCII (`>= 0x80`).
/// - Devuelve `Err(NormalizeAsciiError::EmptyAfterTrimming)` si después de recortar separadores al
///   inicio/fin, la entrada queda vacía.
/// - Sustituye cualquier secuencia de separadores por un único espacio `' '`.
/// - El resultado queda siempre en minúsculas.
///
/// Intenta devolver siempre `Cow::Borrowed` para no reservar memoria, y `Cow::Owned` sólo si ha
/// tenido que aplicar cambios para normalizar.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::util;
/// assert_eq!(util::normalize_ascii("  Foo\tBAR  CLi\r\n").unwrap().as_ref(), "foo bar cli");
/// ```
pub fn normalize_ascii(input: &str) -> Result<Cow<'_, str>, NormalizeAsciiError> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(NormalizeAsciiError::IsEmpty);
    }

    // Primera pasada, determina si se necesita asignación y calcula los límites del contenido.
    let mut start = 0;
    let mut end = 0;

    let mut needs_alloc = false;
    let mut needs_alloc_ws = false;
    let mut has_content = false;
    let mut prev_sep = false;

    for (pos, &b) in bytes.iter().enumerate() {
        if !b.is_ascii() {
            return Err(NormalizeAsciiError::NonAscii);
        }
        if b.is_ascii_whitespace() {
            if has_content {
                if b != b' ' || prev_sep {
                    needs_alloc_ws = true;
                }
                prev_sep = true;
            }
        } else {
            if needs_alloc_ws {
                needs_alloc = true;
                needs_alloc_ws = false;
            }
            if b.is_ascii_uppercase() {
                needs_alloc = true;
            }
            prev_sep = false;
            if !has_content {
                start = pos;
                has_content = true;
            }
            end = pos + 1;
        }
    }

    if !has_content {
        return Err(NormalizeAsciiError::EmptyAfterTrimming);
    }

    let slice = &input[start..end];

    if !needs_alloc {
        return Ok(Cow::Borrowed(slice));
    }

    // Segunda pasada, construye la cadena normalizada.
    let mut output = String::with_capacity(slice.len());
    let mut prev_sep = true;

    for &b in slice.as_bytes() {
        if b.is_ascii_whitespace() {
            if !prev_sep {
                output.push(' ');
                prev_sep = true;
            }
        } else {
            output.push(b.to_ascii_lowercase() as char);
            prev_sep = false;
        }
    }

    Ok(Cow::Owned(output))
}

/// Normaliza una cadena ASCII, opcionalmente vacía, con uno o varios tokens separados.
///
/// - Devuelve `Some(Cow)` si la entrada es válida ASCII (normalizada a minúsculas).
/// - Devuelve `Some(Cow::Borrowed(""))` si la entrada es `""` o queda vacía tras recortar.
/// - Devuelve `None` si la entrada contiene bytes no ASCII; y emite un `trace::debug!` con el campo
///   `target`.
#[inline]
pub fn normalize_ascii_or_empty<'a>(input: &'a str, target: &'static str) -> Option<Cow<'a, str>> {
    match normalize_ascii(input) {
        Ok(s) => Some(s),
        Err(NormalizeAsciiError::NonAscii) => {
            trace::debug!(
                target = %target,
                input = %input.escape_default(),
                "Ignoring due to non-ASCII chars"
            );
            None
        }
        Err(NormalizeAsciiError::IsEmpty | NormalizeAsciiError::EmptyAfterTrimming) => {
            Some(Cow::Borrowed(""))
        }
    }
}

/// Recorta espacios, convierte una cadena vacía en `None` y normaliza el resto.
///
/// Convierte en un único token: en minúsculas y con cada espacio en blanco sustituido por `_`.
///
/// Al contrario que [`normalize_ascii()`], no colapsa las secuencias de varios espacios en blanco
/// seguidos, sino que cada uno se convierte en su propio `_` (p. ej. dos espacios seguidos generan
/// `__`).
///
/// Está pensada para identificadores internos (p. ej. el `id` HTML de un componente, o la clave
/// `referer_id` de una acción) construidos a partir de texto libre.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::util;
/// assert_eq!(util::normalize_token("  My Id  "), Some("my_id".to_string()));
/// assert_eq!(util::normalize_token("AÑO\tNuevo"), Some("año_nuevo".to_string()));
/// assert_eq!(util::normalize_token("a   b"), Some("a___b".to_string()));
/// assert_eq!(util::normalize_token("   "), None);
/// assert_eq!(util::normalize_token(""), None);
/// ```
#[inline]
pub fn normalize_token(s: impl AsRef<str>) -> Option<String> {
    non_blank(s.as_ref()).map(|s| s.to_lowercase().replace(char::is_whitespace, "_"))
}

/// Indica si una URL **parece** externa por su prefijo.
///
/// No es una validación de la URL; sólo mira el inicio del texto: `//` (relativa al protocolo),
/// `http://`, `https://`, `mailto:` o `tel:`. La comparación ignora mayúsculas/minúsculas ASCII en
/// los prefijos (`HTTPS://...` o `MAILTO:...` se detectan igual), porque el esquema de una URI es
/// *case-insensitive* según RFC 3986.
///
/// La usan [`RoutePath::is_external()`](crate::html::RoutePath::is_external) y las conversiones
/// `From<&str>`/`From<String>` de [`Route`](crate::core::component::Route) para decidir si una
/// ruta debe evitar [`Context::route()`](crate::core::component::Context::route).
///
/// Cualquier otro código que necesite el mismo criterio (por ejemplo, para decidir si un enlace
/// debe llevar `target="_blank"`) puede usarla en lugar de reimplementar su propia versión.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::util;
/// assert!(util::url_looks_external("https://example.com"));
/// assert!(util::url_looks_external("mailto:info@example.com"));
/// assert!(util::url_looks_external("HTTPS://EXAMPLE.COM"));
/// assert!(!util::url_looks_external("/admin/users"));
/// ```
pub fn url_looks_external(url: &str) -> bool {
    starts_with_ignore_ascii_case(url, "//")
        || starts_with_ignore_ascii_case(url, "http://")
        || starts_with_ignore_ascii_case(url, "https://")
        || starts_with_ignore_ascii_case(url, "mailto:")
        || starts_with_ignore_ascii_case(url, "tel:")
}

// Compara si `text` empieza por `prefix` ignorando mayúsculas/minúsculas en ASCII (los esquemas de
// URI son case-insensitive según RFC 3986). No reserva memoria: sólo compara el primer tramo de
// bytes de `text` con `prefix`.
fn starts_with_ignore_ascii_case(text: &str, prefix: &str) -> bool {
    text.get(..prefix.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
}

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
/// # use pagetop::prelude::*;
/// // Ruta relativa, se resuelve respecto a CARGO_MANIFEST_DIR o al directorio actual (`cwd`).
/// println!("{:#?}", util::resolve_absolute_dir("documents"));
///
/// // Ruta absoluta, se normaliza y valida tal cual.
/// println!("{:#?}", util::resolve_absolute_dir("/var/www"));
/// ```
pub fn resolve_absolute_dir<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    resolve_absolute_dir_with_base(path, env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from))
}

/// Auxiliar de [`resolve_absolute_dir`] expuesta para tests.
///
/// Permite probar la lógica de resolución inyectando el directorio base explícitamente, sin
/// modificar variables de entorno globales. No forma parte de la API pública.
#[doc(hidden)]
pub fn resolve_absolute_dir_with_base<P: AsRef<Path>>(
    path: P,
    base: Option<PathBuf>,
) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Directorio base proporcionado, o current_dir() en su defecto.
        base.or_else(|| env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."))
            .join(path)
    };

    // Resuelve `.`/`..`, enlaces simbólicos y obtiene la ruta absoluta en un único paso.
    let absolute_dir = candidate.canonicalize()?;

    // Asegura que realmente es un directorio existente.
    if absolute_dir.is_dir() {
        Ok(absolute_dir)
    } else {
        let msg = format!("path \"{}\" is not a directory", absolute_dir.display());
        trace::warn!(msg);
        Err(io::Error::new(io::ErrorKind::InvalidInput, msg))
    }
}
