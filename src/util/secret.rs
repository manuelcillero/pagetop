use serde::{Deserialize, Deserializer};
use zeroize::Zeroize;

use std::fmt;

/// Un secreto, como una contraseña, que no se puede imprimir por descuido.
///
/// Sirve tanto para un valor de la configuración (la contraseña de la base de datos o del servidor
/// de correo) como para uno tecleado por el usuario en un formulario. No implementa `Display`, y
/// `Debug` escribe `***`, así que no puede acabar en una traza, un mensaje de error o un `{:?}` de
/// los ajustes.
///
/// El valor sólo se lee de forma explícita, con [`expose()`](Self::expose), y se borra de la
/// memoria al descartarse.
///
/// Eso sí, sólo se borra el valor que guarda el tipo. Las copias que se hicieron antes de crearlo
/// (al leer el archivo de configuración o el cuerpo de la petición, por ejemplo) quedan fuera de su
/// alcance.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let secret = util::SecretStr::new("hunter2");
/// assert_eq!(format!("{secret:?}"), "***");
/// assert_eq!(secret.expose(), "hunter2");
/// ```
#[derive(Clone, Default)]
pub struct SecretStr(String);

impl SecretStr {
    /// Crea el secreto a partir de su valor en claro.
    pub fn new(value: impl Into<String>) -> Self {
        SecretStr(value.into())
    }

    /// Devuelve el valor en claro, para usarlo donde hace falta.
    pub fn expose(&self) -> &str {
        &self.0
    }

    /// Devuelve si está vacío.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Longitud en bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Debug for SecretStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("***")
    }
}

impl Drop for SecretStr {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl<'de> Deserialize<'de> for SecretStr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(SecretStr)
    }
}
