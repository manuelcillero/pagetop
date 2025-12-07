//! Macros y funciones útiles.

use crate::trace;

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
