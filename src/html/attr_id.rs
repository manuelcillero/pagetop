use crate::{builder_fn, AutoDefault};

/// Identificador normalizado para el atributo `id` o similar de HTML.
///
/// Este tipo encapsula `Option<String>` garantizando un valor normalizado para su uso:
///
/// - Se eliminan los espacios al principio y al final.
/// - Se convierte a minúsculas.
/// - Se sustituyen los espacios intermedios por guiones bajos (`_`).
/// - Si el resultado es una cadena vacía, se guarda `None`.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let id = AttrId::new("  main Section ");
/// assert_eq!(id.as_str(), Some("main_section"));
///
/// let empty = AttrId::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug, Hash, Eq, PartialEq)]
pub struct AttrId(Option<String>);

impl AttrId {
    /// Crea un nuevo `AttrId` normalizando el valor.
    pub fn new(value: impl AsRef<str>) -> Self {
        AttrId::default().with_value(value)
    }

    // **< AttrId BUILDER >*************************************************************************

    /// Establece un identificador nuevo normalizando el valor.
    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        let value = value.as_ref().trim().to_ascii_lowercase().replace(' ', "_");
        self.0 = if value.is_empty() { None } else { Some(value) };
        self
    }

    // **< AttrId GETTERS >*************************************************************************

    /// Devuelve el identificador normalizado, si existe.
    pub fn get(&self) -> Option<String> {
        self.0.as_ref().cloned()
    }

    /// Devuelve el identificador normalizado (sin clonar), si existe.
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_deref()
    }

    /// Devuelve el identificador normalizado (propiedad), si existe.
    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    /// `true` si no hay valor.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}
