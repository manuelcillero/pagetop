use crate::html::Markup;
use crate::locale::{L10n, LangId};
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
///     hello.using(&LangMatch::resolve("es-ES")),
///     Some(String::from("¡Hola mundo!"))
/// );
///
/// // Japonés no disponible, traduce al idioma de respaldo ("en-US").
/// assert_eq!(
///     hello.using(&LangMatch::resolve("ja-JP")),
///     Some(String::from("Hello world!"))
/// );
///
/// // Para incrustar en HTML escapado:
/// let markup = hello.to_markup(&LangMatch::resolve("es-ES"));
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

    /// Devuelve la traducción para `language`, si existe.
    pub fn using(&self, language: &impl LangId) -> Option<String> {
        self.0.using(language)
    }

    /// Devuelve la traducción *escapada* como [`Markup`] para `language`, si existe.
    ///
    /// Útil para incrustar el texto directamente en plantillas HTML sin riesgo de inyección de
    /// contenido.
    pub fn to_markup(&self, language: &impl LangId) -> Markup {
        self.0.to_markup(language)
    }
}
