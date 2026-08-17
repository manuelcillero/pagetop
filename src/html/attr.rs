use crate::{AutoDefault, builder_fn, util};

// **< AttrName >***********************************************************************************

/// Nombre normalizado para el atributo `name` o similar de HTML.
///
/// Este tipo encapsula `Option<String>` garantizando un valor normalizado para su uso:
///
/// - Se eliminan los espacios al principio y al final.
/// - Se convierte a minúsculas.
/// - Se sustituyen los espacios (`' '`) intermedios por guiones bajos (`_`).
/// - Si el resultado es una cadena vacía, se guarda `None`.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let name = AttrName::new("  DISplay name ");
/// assert_eq!(name.as_deref(), Some("display_name"));
///
/// let empty = AttrName::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct AttrName(Option<String>);

impl AttrName {
    /// Crea un nuevo `AttrName` normalizando el valor.
    pub fn new(name: impl AsRef<str>) -> Self {
        Self::default().with_name(name)
    }

    // **< AttrName BUILDER >***********************************************************************

    /// Establece un nombre nuevo normalizando el valor.
    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.0 = util::normalize_token(name);
        self
    }

    // **< AttrName GETTERS >***********************************************************************

    /// Devuelve el nombre normalizado (sin clonar), si existe.
    pub fn as_deref(&self) -> Option<&str> {
        self.0.as_deref()
    }

    /// Devuelve el nombre normalizado (clonado), si existe.
    pub fn get(&self) -> Option<String> {
        self.0.clone()
    }

    /// `true` si no hay valor.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

// **< AttrValue >**********************************************************************************

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
/// assert_eq!(s.as_deref(), Some("a new string"));
///
/// let empty = AttrValue::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct AttrValue(Option<String>);

impl AttrValue {
    /// Crea un nuevo `AttrValue` normalizando el valor.
    pub fn new(value: impl AsRef<str>) -> Self {
        Self::default().with_str(value)
    }

    // **< AttrValue BUILDER >**********************************************************************

    /// Establece una cadena nueva normalizando el valor.
    #[builder_fn]
    pub fn with_str(mut self, value: impl AsRef<str>) -> Self {
        self.0 = util::non_blank(value.as_ref()).map(str::to_string);
        self
    }

    // **< AttrValue GETTERS >**********************************************************************

    /// Devuelve la cadena normalizada (sin clonar), si existe.
    pub fn as_deref(&self) -> Option<&str> {
        self.0.as_deref()
    }

    /// Devuelve la cadena normalizada (clonada), si existe.
    pub fn get(&self) -> Option<String> {
        self.0.clone()
    }

    /// `true` si no hay valor.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}
