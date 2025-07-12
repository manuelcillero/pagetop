//! Funciones y macros útiles.

use crate::trace;

use std::io;
use std::path::{Path, PathBuf};


/// Devuelve la ruta absoluta a un directorio existente.
///
/// * Si `relative_path` es una ruta absoluta, entonces se ignora `root_path`.
/// * Si la ruta final es relativa, se convierte en absoluta respecto al directorio actual.
/// * Devuelve error si la ruta no existe o no es un directorio.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let root = "/home/user";
/// let rel  = "documents";
/// println!("{:#?}", util::absolute_dir(root, rel));
/// ```
pub fn absolute_dir<P, Q>(root_path: P, relative_path: Q) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    // Une ambas rutas:
    // - Si `relative_path` es absoluta, el `join` la devuelve tal cual, descartando `root_path`.
    // - Si el resultado es aún relativo, lo será respecto al directorio actual.
    let candidate = root_path.as_ref().join(relative_path.as_ref());

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

// MACROS ÚTILES ***********************************************************************************

#[doc(hidden)]
pub use paste::paste;

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
