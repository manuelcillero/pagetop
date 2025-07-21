use crate::{builder_fn, AutoDefault};

/// Cadena normalizada para renderizar en atributos HTML.
///
/// Este tipo encapsula `Option<String>` garantizando un valor normalizado para su uso.
///
/// # Normalización
/// - Se eliminan los espacios al principio y al final.
/// - Si el resultado es una cadena vacía, se guarda `None`.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// let s = OptionString::new("  a new string   ");
/// assert_eq!(s.get(), Some(String::from("a new string")));
///
/// let empty = OptionString::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug, Hash, Eq, PartialEq)]
pub struct OptionString(Option<String>);

impl OptionString {
    /// Crea un nuevo [`OptionString`].
    ///
    /// El valor se normaliza automáticamente.
    pub fn new(value: impl AsRef<str>) -> Self {
        OptionString::default().with_value(value)
    }

    // OptionString BUILDER ************************************************************************

    /// Establece una cadena nueva.
    ///
    /// El valor se normaliza automáticamente.
    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        let value = value.as_ref().trim().to_owned();
        self.0 = (!value.is_empty()).then_some(value);
        self
    }

    // OptionString GETTERS ************************************************************************

    /// Devuelve la cadena, si existe.
    pub fn get(&self) -> Option<String> {
        if let Some(value) = &self.0 {
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
        None
    }
}
