use crate::html::Markup;
use crate::locale::{L10n, LangId};
use crate::{builder_fn, AutoDefault};

/// Texto para [traducir](crate::locale) en atributos HTML.
///
/// Encapsula un [`L10n`] para manejar traducciones de forma segura en atributos.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Traducción por clave en las locales por defecto de PageTop.
/// let hello = AttrL10n::new(L10n::l("test-hello-world"));
///
/// // Español disponible.
/// assert_eq!(
///     hello.lookup(&LangMatch::resolve("es-ES")),
///     Some("¡Hola mundo!".to_string())
/// );
///
/// // Japonés no disponible, traduce al idioma de respaldo ("en-US").
/// assert_eq!(
///     hello.lookup(&LangMatch::resolve("ja-JP")),
///     Some("Hello world!".to_string())
/// );
///
/// // Uso típico en un atributo:
/// let title = hello.value(&LangMatch::resolve("es-ES"));
/// // Ejemplo: html! { a title=(title) { "Link" } }
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct AttrL10n(L10n);

impl AttrL10n {
    /// Crea una nueva instancia `AttrL10n`.
    pub fn new(value: L10n) -> Self {
        AttrL10n(value)
    }

    // **< AttrL10n BUILDER >***********************************************************************

    /// Establece una traducción nueva.
    #[builder_fn]
    pub fn with_value(mut self, value: L10n) -> Self {
        self.0 = value;
        self
    }

    // **< AttrL10n GETTERS >***********************************************************************

    /// Devuelve la traducción para `language`, si existe.
    pub fn lookup(&self, language: &impl LangId) -> Option<String> {
        self.0.lookup(language)
    }

    /// Devuelve la traducción para `language` o una cadena vacía si no existe.
    pub fn value(&self, language: &impl LangId) -> String {
        self.0.lookup(language).unwrap_or_default()
    }

    /// **Obsoleto desde la versión 0.4.0**: no recomendado para atributos HTML.
    #[deprecated(since = "0.4.0", note = "For attributes use `lookup()` or `value()`")]
    pub fn to_markup(&self, language: &impl LangId) -> Markup {
        self.0.using(language)
    }
}
