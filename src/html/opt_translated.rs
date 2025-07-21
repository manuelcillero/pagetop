use crate::html::Markup;
use crate::locale::{L10n, LanguageIdentifier};
use crate::{builder_fn, AutoDefault};

/// Cadena para traducir al renderizar ([`locale`](crate::locale)).
///
/// Encapsula un tipo [`L10n`] para manejar traducciones de forma segura.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Traducción por clave en las locales por defecto de PageTop.
/// let hello = OptionTranslated::new(L10n::l("test-hello-world"));
///
/// // Español disponible.
/// assert_eq!(
///     hello.using(LangMatch::langid_or_default("es-ES")),
///     Some(String::from("¡Hola mundo!"))
/// );
///
/// // Japonés no disponible, traduce al idioma de respaldo ("en-US").
/// assert_eq!(
///     hello.using(LangMatch::langid_or_fallback("ja-JP")),
///     Some(String::from("Hello world!"))
/// );
///
/// // Para incrustar en HTML escapado:
/// let markup = hello.escaped(LangMatch::langid_or_default("es-ES"));
/// assert_eq!(markup.into_string(), "¡Hola mundo!");
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct OptionTranslated(L10n);

impl OptionTranslated {
    /// Crea una nueva instancia [`OptionTranslated`].
    pub fn new(value: L10n) -> Self {
        OptionTranslated(value)
    }

    // OptionTranslated BUILDER ********************************************************************

    /// Establece una traducción nueva.
    #[builder_fn]
    pub fn with_value(mut self, value: L10n) -> Self {
        self.0 = value;
        self
    }

    // OptionTranslated GETTERS ********************************************************************

    /// Devuelve la traducción para `langid`, si existe.
    pub fn using(&self, langid: &LanguageIdentifier) -> Option<String> {
        self.0.using(langid)
    }

    /// Devuelve la traducción *escapada* como [`Markup`] para `langid`, si existe.
    ///
    /// Útil para incrustar el texto directamente en plantillas HTML sin riesgo de inyección de
    /// contenido.
    pub fn escaped(&self, langid: &LanguageIdentifier) -> Markup {
        self.0.escaped(langid)
    }
}
