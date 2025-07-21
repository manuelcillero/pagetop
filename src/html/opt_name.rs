use crate::{builder_fn, AutoDefault};

/// Nombre normalizado para el atributo `name` o similar de HTML.
///
/// Este tipo encapsula `Option<String>` garantizando un valor normalizado para su uso.
///
/// # Normalización
/// - Se eliminan los espacios al principio y al final.
/// - Se sustituyen los espacios intermedios por guiones bajos (`_`).
/// - Si el resultado es una cadena vacía, se guarda `None`.
///
/// ## Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// let name = OptionName::new("  display name ");
/// assert_eq!(name.get(), Some(String::from("display_name")));
///
/// let empty = OptionName::default();
/// assert_eq!(empty.get(), None);
/// ```
#[derive(AutoDefault, Clone, Debug, Hash, Eq, PartialEq)]
pub struct OptionName(Option<String>);

impl OptionName {
    /// Crea un nuevo [`OptionName`].
    ///
    /// El valor se normaliza automáticamente.
    pub fn new(value: impl AsRef<str>) -> Self {
        OptionName::default().with_value(value)
    }

    // OptionName BUILDER **************************************************************************

    /// Establece un nombre nuevo.
    ///
    /// El valor se normaliza automáticamente.
    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        let value = value.as_ref().trim().replace(' ', "_");
        self.0 = (!value.is_empty()).then_some(value);
        self
    }

    // OptionName GETTERS **************************************************************************

    /// Devuelve el nombre, si existe.
    pub fn get(&self) -> Option<String> {
        if let Some(value) = &self.0 {
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
        None
    }
}
