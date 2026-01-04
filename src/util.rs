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
pub fn normalize_ascii<'a>(input: &'a str) -> Result<Cow<'a, str>, NormalizeAsciiError> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(NormalizeAsciiError::IsEmpty);
    }

    let mut start = 0usize;
    let mut end = 0usize;

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
