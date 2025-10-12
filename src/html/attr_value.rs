use crate::{builder_fn, AutoDefault};

/// Cadena normalizada para renderizar en atributos HTML.
///
/// Este tipo encapsula `Option<String>` garantizando un valor normalizado para su uso:
///
/// - Se eliminan los espacios al principio y al final.
/// - Si el resultado es una cadena vacía, se guarda `None`.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let s = AttrValue::new("  a new string   ");
/// assert_eq!(s.as_str(), Some("a new string"));
///
/// let empty = AttrValue::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug, Hash, Eq, PartialEq)]
pub struct AttrValue(Option<String>);

impl AttrValue {
    /// Crea un nuevo `AttrValue` normalizando el valor.
    pub fn new(value: impl AsRef<str>) -> Self {
        AttrValue::default().with_value(value)
    }

    // **< AttrValue BUILDER >**********************************************************************

    /// Establece una cadena nueva normalizando el valor.
    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        let value = value.as_ref().trim();
        self.0 = if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        };
        self
    }

    // **< AttrValue GETTERS >**********************************************************************

    /// Devuelve la cadena normalizada, si existe.
    pub fn get(&self) -> Option<String> {
        self.0.as_ref().cloned()
    }

    /// Devuelve la cadena normalizada (sin clonar), si existe.
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_deref()
    }

    /// Devuelve la cadena normalizada (propiedad), si existe.
    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    /// `true` si no hay valor.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}
