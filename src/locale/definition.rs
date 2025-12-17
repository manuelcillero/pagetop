use crate::{global, trace};

use super::languages::LANGUAGES;
use super::{langid, LanguageIdentifier};

use std::sync::LazyLock;

// Identificador del idioma configurado para la aplicación, si es válido.
static CONFIG_LANGID: LazyLock<Option<&'static LanguageIdentifier>> = LazyLock::new(|| {
    Locale::resolve(global::SETTINGS.app.language.as_deref().unwrap_or("")).as_option()
});

// Identificador del idioma de respaldo (predefinido a `"en-US"`).
static FALLBACK_LANGID: LazyLock<LanguageIdentifier> = LazyLock::new(|| langid!("en-US"));

/// Representa el identificador de idioma [`LanguageIdentifier`] asociado a un recurso.
///
/// Este *trait* permite que distintas estructuras expongan su idioma de forma uniforme. Las
/// implementaciones deben garantizar que siempre se devuelve un identificador de idioma válido. Si
/// el recurso no tiene uno asignado, se puede devolver, si procede, el identificador de idioma por
/// defecto de la aplicación ([`Locale::default_langid()`]).
pub trait LangId {
    /// Devuelve el identificador de idioma asociado al recurso.
    fn langid(&self) -> &'static LanguageIdentifier;
}

/// Resultado de resolver un identificador de idioma.
///
/// Utiliza [`Locale::resolve()`] para transformar una cadena de idioma en un [`LanguageIdentifier`]
/// soportado por PageTop.
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Coincidencia exacta.
/// let lang = Locale::resolve("es-ES");
/// assert_eq!(lang.langid().to_string(), "es-ES");
///
/// // Coincidencia parcial (retrocede al idioma base si no hay variante regional).
/// let lang = Locale::resolve("es-EC");
/// assert_eq!(lang.langid().to_string(), "es-ES"); // Porque "es-EC" no está soportado.
///
/// // Idioma no especificado.
/// let lang = Locale::resolve("");
/// assert_eq!(lang, Locale::Unspecified);
///
/// // Idioma no soportado.
/// let lang = Locale::resolve("ja-JP");
/// assert_eq!(lang, Locale::Unsupported("ja-JP".to_string()));
/// ```
///
/// Con la siguiente instrucción siempre se obtiene un [`LanguageIdentifier`] válido, ya sea porque
/// resuelve un idioma soportado o porque se aplica el idioma por defecto o, en último término, el
/// de respaldo (`"en-US"`):
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Idioma por defecto si no resuelve.
/// let lang = Locale::resolve("it-IT");
/// let langid = lang.langid();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Locale {
    /// No se ha especificado ningún identificador de idioma.
    ///
    /// Se usa cuando la cadena de idioma está vacía o no se puede obtener un idioma válido de la
    /// petición HTTP.
    Unspecified,
    /// El identificador se ha resuelto a un idioma soportado por PageTop.
    ///
    /// Se utiliza cuando se encuentra un [`LanguageIdentifier`] en la lista de idiomas soportados
    /// por PageTop que coincide exactamente con el identificador de idioma (p. ej. `"es-ES"`) o
    /// con el identificador del idioma base (p. ej. `"es"`).
    Resolved(&'static LanguageIdentifier),
    /// El identificador de idioma no está soportado por PageTop.
    Unsupported(String),
}

impl Default for Locale {
    /// Resuelve al idioma por defecto y, si no está disponible, al idioma de respaldo (`"en-US"`).
    fn default() -> Self {
        Locale::Resolved(Locale::default_langid())
    }
}

impl Locale {
    /// Resuelve `language` y devuelve la variante [`Locale`] apropiada.
    ///
    /// - Si la cadena está vacía o contiene solo espacios, devuelve [`Locale::Unspecified`].
    /// - Si el idioma se reconoce (ya sea como código completo o como idioma base), devuelve
    ///   [`Locale::Resolved`].
    /// - En caso contrario, devuelve [`Locale::Unsupported`] con la cadena original.
    pub fn resolve(language: impl AsRef<str>) -> Self {
        let language = language.as_ref().trim();

        // Rechaza cadenas vacías.
        if language.is_empty() {
            return Self::Unspecified;
        }

        // Intenta aplicar coincidencia exacta con el código completo (p. ej. "es-MX").
        let lang = language.to_ascii_lowercase();
        if let Some(langid) = LANGUAGES.get(lang.as_str()).map(|(langid, _)| langid) {
            return Self::Resolved(langid);
        }

        // Si la variante regional no existe, retrocede al idioma base (p. ej. "es").
        if let Some((base_lang, _)) = lang.split_once('-') {
            if let Some(langid) = LANGUAGES.get(base_lang).map(|(langid, _)| langid) {
                return Self::Resolved(langid);
            }
        }

        // En caso contrario, indica que el idioma no está soportado.
        Self::Unsupported(language.to_string())
    }

    /// Devuelve el [`LanguageIdentifier`] si el idioma fue reconocido.
    ///
    /// Solo retorna `Some` si la variante es [`Locale::Resolved`]. En cualquier otro caso (por
    /// ejemplo, si el identificador es vacío o no está soportado), devuelve `None`.
    ///
    /// Este método es útil cuando se desea acceder directamente al idioma reconocido sin aplicar el
    /// idioma por defecto ni el de respaldo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let lang = Locale::resolve("es-ES").as_option();
    /// assert_eq!(lang.unwrap().to_string(), "es-ES");
    ///
    /// let lang = Locale::resolve("ja-JP").as_option();
    /// assert!(lang.is_none());
    /// ```
    #[inline]
    pub fn as_option(&self) -> Option<&'static LanguageIdentifier> {
        match self {
            Locale::Resolved(l) => Some(l),
            _ => None,
        }
    }

    // **< Locale HELPERS >*************************************************************************

    /// Inicializa el idioma por defecto que utilizará la aplicación.
    ///
    /// Debe llamarse durante la inicialización para indicar si el idioma por defecto procede de la
    /// configuración, de una configuración no válida o del idioma de respaldo.
    pub(crate) fn init() {
        match global::SETTINGS.app.language.as_deref() {
            Some(raw) if !raw.trim().is_empty() => {
                if let Some(langid) = *CONFIG_LANGID {
                    trace::debug!("Default language \"{langid}\" (from config: \"{raw}\")");
                } else {
                    trace::debug!(
                        "Default language \"{}\" (fallback, invalid config: \"{raw}\")",
                        *FALLBACK_LANGID
                    );
                }
            }
            _ => trace::debug!(
                "Default language \"{}\" (fallback, no config)",
                *FALLBACK_LANGID
            ),
        }
    }

    /// Devuelve el identificador de idioma configurado explícitamente, si es válido.
    ///
    /// Si no se ha configurado un idioma por defecto o el valor no es válido, devuelve `None`.
    pub fn configured_langid() -> Option<&'static LanguageIdentifier> {
        *CONFIG_LANGID
    }

    /// Devuelve siempre el identificador de idioma de respaldo (`"en-US"`).
    ///
    /// Es el idioma garantizado incluso cuando no haya configuración de la aplicación o cuando
    /// el valor configurado no sea válido.
    pub fn fallback_langid() -> &'static LanguageIdentifier {
        &FALLBACK_LANGID
    }

    /// Devuelve el identificador de idioma configurado o, en su defecto, el de respaldo.
    ///
    /// Este es el idioma que utiliza internamente [`Locale::default()`] y resulta útil como idioma
    /// base cuando no se dispone de un contexto más específico.
    pub fn default_langid() -> &'static LanguageIdentifier {
        (*CONFIG_LANGID).unwrap_or(&FALLBACK_LANGID)
    }
}

/// Permite a [`Locale`] actuar como proveedor de idioma.
///
/// Devuelve el [`LanguageIdentifier`] si la variante es [`Locale::Resolved`]; en caso contrario,
/// devuelve el idioma por defecto de la aplicación y, si tampoco está disponible, el idioma de
/// respaldo (`"en-US"`).
///
/// Resulta útil para usar un valor de [`Locale`] como fuente de traducción en
/// [`L10n::lookup()`](crate::locale::L10n::lookup) o [`L10n::using()`](crate::locale::L10n::using).
impl LangId for Locale {
    #[inline]
    fn langid(&self) -> &'static LanguageIdentifier {
        match self {
            Locale::Resolved(l) => l,
            _ => Locale::default_langid(),
        }
    }
}
