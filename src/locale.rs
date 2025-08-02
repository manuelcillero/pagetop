//! Localización (L10n).
//!
//! `PageTop` utiliza las especificaciones de [Fluent](https://www.projectfluent.org/) para la
//! localización de aplicaciones, y aprovecha [fluent-templates](https://docs.rs/fluent-templates/)
//! para integrar los recursos de traducción directamente en el binario de la aplicación.
//!
//! # Sintaxis Fluent (FTL)
//!
//! El formato empleado para describir los recursos de traducción se denomina
//! [FTL](https://www.projectfluent.org/fluent/guide/). Está diseñado para ser legible y expresivo,
//! permitiendo representar construcciones complejas del lenguaje natural como el género, el plural
//! o las conjugaciones verbales.
//!
//! # Recursos Fluent
//!
//! Por defecto las traducciones están en el directorio `src/locale`, con subdirectorios para cada
//! [Identificador de Idioma Unicode](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier)
//! válido. Podríamos tener una estructura como esta:
//!
//! ```text
//! src/locale/
//!      ├── common.ftl
//!      ├── en-US/
//!      │   ├── default.ftl
//!      │   └── main.ftl
//!      ├── es-ES/
//!      │   ├── default.ftl
//!      │   └── main.ftl
//!      ├── es-MX/
//!      │   ├── default.ftl
//!      │   └── main.ftl
//!      └── fr-FR/
//!          ├── default.ftl
//!          └── main.ftl
//! ```
//!
//! Ejemplo de un archivo en `src/locale/en-US/main.ftl`
//!
//! ```text
//! hello-world = Hello world!
//! hello-user = Hello, {$userName}!
//! shared-photos =
//!     {$userName} {$photoCount ->
//!         [one] added a new photo
//!        *[other] added {$photoCount} new photos
//!     } of {$userGender ->
//!         [male] him and his family
//!         [female] her and her family
//!        *[other] the family
//!     }.
//! ```
//!
//! Y su archivo equivalente para español en `src/locale/es-ES/main.ftl`:
//!
//! ```text
//! hello-world = Hola mundo!
//! hello-user = ¡Hola, {$userName}!
//! shared-photos =
//!     {$userName} {$photoCount ->
//!         [one] ha añadido una nueva foto
//!        *[other] ha añadido {$photoCount} nuevas fotos
//!     } de {$userGender ->
//!         [male] él y su familia
//!         [female] ella y su familia
//!        *[other] la familia
//!     }.
//! ```
//!
//!
//! # Cómo aplicar la localización en tu código
//!
//! Una vez creado el directorio con los recursos FTL, basta con usar la macro
//! [`include_locales!`](crate::include_locales) para integrarlos en la aplicación.
//!
//! Si los recursos se encuentran en el directorio por defecto `src/locale` del *crate*, sólo hay
//! que declarar:
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! include_locales!(LOCALES_SAMPLE);
//! ```
//!
//! Si están ubicados en otro directorio se puede usar la forma:
//!
//! ```rust,ignore
//! include_locales!(LOCALES_SAMPLE from "ruta/a/las/traducciones");
//! ```
//!
//! Y *voilà*, sólo queda operar con los idiomas soportados por `PageTop` usando [`LangMatch`] y
//! traducir textos con [`L10n`].

use crate::html::{Markup, PreEscaped, Render};
use crate::{global, hm, AutoDefault};

pub use fluent_templates;
pub use unic_langid::{CharacterDirection, LanguageIdentifier};

use unic_langid::langid;

use fluent_templates::Loader;
use fluent_templates::StaticLoader as Locales;

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;

use std::fmt;

// Asocia cada identificador de idioma (como "en-US") con su respectivo [`LanguageIdentifier`] y la
// clave en *locale/.../languages.ftl* para obtener el nombre del idioma según la localización.
static LANGUAGES: LazyLock<HashMap<&str, (LanguageIdentifier, &str)>> = LazyLock::new(|| {
    hm![
        "en"    => ( langid!("en-US"), "english" ),
        "en-GB" => ( langid!("en-GB"), "english_british" ),
        "en-US" => ( langid!("en-US"), "english_united_states" ),
        "es"    => ( langid!("es-ES"), "spanish" ),
        "es-ES" => ( langid!("es-ES"), "spanish_spain" ),
    ]
});

// Identificador de idioma de **respaldo** (predefinido a `en-US`).
//
// Se usa cuando el valor del identificador de idioma en las traducciones no corresponde con ningún
// idioma soportado por la aplicación.
static FALLBACK_LANGID: LazyLock<LanguageIdentifier> = LazyLock::new(|| langid!("en-US"));

// Identificador de idioma **por defecto** para la aplicación.
//
// Se resuelve a partir de [`global::SETTINGS.app.language`](global::SETTINGS). Si el identificador
// de idioma no es válido o no está disponible entonces resuelve como [`FALLBACK_LANGID`].
pub(crate) static DEFAULT_LANGID: LazyLock<&LanguageIdentifier> =
    LazyLock::new(|| LangMatch::langid_or_fallback(&global::SETTINGS.app.language));

/// Operaciones con los idiomas soportados por `PageTop`.
///
/// Utiliza [`LangMatch`] para transformar un identificador de idioma en un [`LanguageIdentifier`]
/// soportado por `PageTop`.
///
/// # Ejemplos
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Coincidencia exacta.
/// let lang = LangMatch::resolve("es-ES");
/// assert_eq!(lang.as_langid().to_string(), "es-ES");
///
/// // Coincidencia parcial (con el idioma base).
/// let lang = LangMatch::resolve("es-EC");
/// assert_eq!(lang.as_langid().to_string(), "es-ES"); // Porque "es-EC" no está soportado.
///
/// // Idioma no especificado.
/// let lang = LangMatch::resolve("");
/// assert_eq!(lang, LangMatch::Unspecified);
///
/// // Idioma no soportado.
/// let lang = LangMatch::resolve("ja-JP");
/// assert_eq!(lang, LangMatch::Unsupported(String::from("ja-JP")));
/// ```
///
/// Las siguientes líneas devuelven siempre un [`LanguageIdentifier`] válido, ya sea porque
/// resuelven un idioma soportado o porque aplican el idioma por defecto o de respaldo:
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Idioma por defecto si no resuelve.
/// let lang = LangMatch::resolve("it-IT");
/// let langid = lang.as_langid();
///
/// // Idioma por defecto si no se encuentra.
/// let langid = LangMatch::langid_or_default("es-MX");
///
/// // Idioma de respaldo ("en-US") si no se encuentra.
/// let langid = LangMatch::langid_or_fallback("es-MX");
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LangMatch {
    /// Cuando el identificador de idioma es una cadena vacía.
    Unspecified,
    /// Si encuentra un [`LanguageIdentifier`] en la lista de idiomas soportados por `PageTop` que
    /// coincide exactamente con el identificador de idioma (p.ej. "es-ES"), o con el identificador
    /// del idioma base (p.ej. "es").
    Found(&'static LanguageIdentifier),
    /// Si el identificador de idioma no está entre los soportados por `PageTop`.
    Unsupported(String),
}

impl LangMatch {
    /// Resuelve `language` y devuelve la variante [`LangMatch`] apropiada.
    pub fn resolve(language: impl AsRef<str>) -> Self {
        let language = language.as_ref().trim();

        // Rechaza cadenas vacías.
        if language.is_empty() {
            return Self::Unspecified;
        }

        // Intenta aplicar coincidencia exacta con el código completo (p.ej. "es-MX").
        if let Some(langid) = LANGUAGES.get(language).map(|(langid, _)| langid) {
            return Self::Found(langid);
        }

        // Si la variante regional no existe, retrocede al idioma base (p.ej. "es").
        if let Some((base_lang, _)) = language.split_once('-') {
            if let Some(langid) = LANGUAGES.get(base_lang).map(|(langid, _)| langid) {
                return Self::Found(langid);
            }
        }

        // En otro caso indica que el idioma no está soportado.
        Self::Unsupported(String::from(language))
    }

    /// Devuelve el idioma de la variante de la instancia, o el idioma por defecto si no está
    /// soportado.
    ///
    /// Siempre devuelve un [`LanguageIdentifier`] válido.
    #[inline]
    pub fn as_langid(&self) -> &'static LanguageIdentifier {
        match self {
            LangMatch::Found(l) => l,
            _ => &DEFAULT_LANGID,
        }
    }

    /// Si `language` está vacío o no está soportado, devuelve el idioma por defecto.
    ///
    /// Siempre devuelve un [`LanguageIdentifier`] válido.
    #[inline]
    pub fn langid_or_default(language: impl AsRef<str>) -> &'static LanguageIdentifier {
        match Self::resolve(language) {
            Self::Found(l) => l,
            _ => &DEFAULT_LANGID,
        }
    }

    /// Si `language` está vacío o no está soportado, devuelve el idioma de respaldo ("en-US").
    ///
    /// Siempre devuelve un [`LanguageIdentifier`] válido.
    #[inline]
    pub fn langid_or_fallback(language: impl AsRef<str>) -> &'static LanguageIdentifier {
        match Self::resolve(language) {
            Self::Found(l) => l,
            _ => &FALLBACK_LANGID,
        }
    }
}

#[macro_export]
/// Define un conjunto de elementos de localización y textos de traducción local.
macro_rules! include_locales {
    // Se eliminan las marcas de aislamiento Unicode en los argumentos para mejorar la legibilidad y
    // la compatibilidad en ciertos contextos de renderizado.
    ( $LOCALES:ident $(, $core_locales:literal)? ) => {
        $crate::locale::fluent_templates::static_loader! {
            static $LOCALES = {
                locales: "src/locale",
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",
                // Elimina marcas de aislamiento Unicode en los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
    ( $LOCALES:ident from $dir_locales:literal $(, $core_locales:literal)? ) => {
        $crate::locale::fluent_templates::static_loader! {
            static $LOCALES = {
                locales: $dir_locales,
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",
                // Elimina marcas de aislamiento Unicode en los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
}

include_locales!(LOCALES_PAGETOP);

// Operación de localización a realizar.
//
// * `None` - No se aplica ninguna localización.
// * `Text` - Con una cadena literal que se devolverá tal cual.
// * `Translate` - Con la clave a resolver en el `Locales` indicado.
#[derive(AutoDefault, Clone, Debug)]
enum L10nOp {
    #[default]
    None,
    Text(String),
    Translate(String),
}

/// Crea instancias para traducir textos localizados.
///
/// Cada instancia puede representar:
///
/// - Un texto puro (`n()`) que no requiere traducción.
/// - Una clave para traducir un texto de las traducciones por defecto de `PageTop` (`l()`).
/// - Una clave para traducir de un conjunto concreto de traducciones (`t()`).
///
/// # Ejemplo
///
/// Los argumentos dinámicos se añaden usando `with_arg()` o `with_args()`.
///
/// ```rust
/// use pagetop::prelude::*;
///
/// // Texto literal sin traducción.
/// let raw = L10n::n("© 2025 PageTop").get();
///
/// // Traducción simple con clave y argumentos.
/// let hello = L10n::l("greeting")
///     .with_arg("name", "Manuel")
///     .get();
/// ```
///
/// También para traducciones a idiomas concretos.
///
/// ```rust,ignore
/// // Traducción con clave, conjunto de traducciones e identificador de idioma a usar.
/// let bye = L10n::t("goodbye", &LOCALES_CUSTOM).using(LangMatch::langid_or_default("it"));
/// ```
#[derive(AutoDefault, Clone)]
pub struct L10n {
    op: L10nOp,
    #[default(&LOCALES_PAGETOP)]
    locales: &'static Locales,
    args: HashMap<String, String>,
}

impl L10n {
    /// **n** = *“native”*. Crea una instancia con una cadena literal sin traducción.
    pub fn n(text: impl Into<String>) -> Self {
        L10n {
            op: L10nOp::Text(text.into()),
            ..Default::default()
        }
    }

    /// **l** = *“lookup”*. Crea una instancia para traducir usando una clave de la tabla de
    /// traducciones por defecto.
    pub fn l(key: impl Into<String>) -> Self {
        L10n {
            op: L10nOp::Translate(key.into()),
            ..Default::default()
        }
    }

    /// **t** = *“translate”*. Crea una instancia para traducir usando una clave de una tabla de
    /// traducciones específica.
    pub fn t(key: impl Into<String>, locales: &'static Locales) -> Self {
        L10n {
            op: L10nOp::Translate(key.into()),
            locales,
            ..Default::default()
        }
    }

    /// Añade un argumento `{$arg}` → `value` a la traducción.
    pub fn with_arg(mut self, arg: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.insert(arg.into(), value.into());
        self
    }

    /// Añade varios argumentos a la traducción de una sola vez (p.ej. usando la macro [`hm!`],
    /// también vec![("k", "v")], incluso un array de duplas u otras colecciones).
    pub fn with_args<I, K, V>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.args
            .extend(args.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Resuelve la traducción usando el idioma por defecto de la aplicación. Devuelve `None` si no
    /// aplica o no encuentra una traducción.
    pub fn get(&self) -> Option<String> {
        self.using(&DEFAULT_LANGID)
    }

    /// Resuelve la traducción usando el [`LanguageIdentifier`] indicado. Devuelve `None` si no
    /// aplica o no encuentra una traducción.
    pub fn using(&self, langid: &LanguageIdentifier) -> Option<String> {
        match &self.op {
            L10nOp::None => None,
            L10nOp::Text(text) => Some(text.to_owned()),
            L10nOp::Translate(key) => self.locales.try_lookup_with_args(
                langid,
                key,
                &self.args.iter().fold(HashMap::new(), |mut arg, (k, v)| {
                    arg.insert(Cow::Owned(k.clone()), v.to_owned().into());
                    arg
                }),
            ),
        }
    }

    /// Traduce y escapa con el [`LanguageIdentifier`] indicado, devolviendo [`Markup`].
    pub fn escaped(&self, langid: &LanguageIdentifier) -> Markup {
        PreEscaped(self.using(langid).unwrap_or_default())
    }
}

impl Render for L10n {
    /// Traduce y escapa con el idioma por defecto, devolviendo [`Markup`].
    fn render(&self) -> Markup {
        PreEscaped(self.get().unwrap_or_default())
    }
}

impl fmt::Debug for L10n {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("L10n")
            .field("op", &self.op)
            .field("args", &self.args)
            // No se puede mostrar `locales`. Se representa con un texto fijo.
            .field("locales", &"<StaticLoader>")
            .finish()
    }
}

impl fmt::Display for L10n {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.op {
            L10nOp::None => String::new(),
            L10nOp::Text(text) => text.clone(),
            L10nOp::Translate(key) => self.get().unwrap_or_else(|| format!("??<{}>", key)),
        };
        write!(f, "{content}")
    }
}
