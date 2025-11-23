//! Localización (L10n).
//!
//! PageTop utiliza las especificaciones de [Fluent](https://www.projectfluent.org/) para la
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
//! Por defecto, las traducciones están en el directorio `src/locale`, con subdirectorios para cada
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
//! Ejemplo de un archivo en `src/locale/en-US/main.ftl`:
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
//! hello-world = ¡Hola, mundo!
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
//! # use pagetop::prelude::*;
//! include_locales!(LOCALES_SAMPLE);
//! ```
//!
//! Si están ubicados en otro directorio, se puede usar la forma:
//!
//! ```rust,ignore
//! include_locales!(LOCALES_SAMPLE from "ruta/a/las/traducciones");
//! ```
//!
//! Y *voilà*, sólo queda operar con los idiomas soportados por PageTop usando [`LangMatch`] y
//! traducir textos con [`L10n`].

use crate::html::{Markup, PreEscaped};
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
        "en-gb" => ( langid!("en-GB"), "english_british" ),
        "en-us" => ( langid!("en-US"), "english_united_states" ),
        "es"    => ( langid!("es-ES"), "spanish" ),
        "es-es" => ( langid!("es-ES"), "spanish_spain" ),
    ]
});

// Identificador de idioma de **respaldo** (predefinido a `en-US`).
//
// Se usa cuando el valor del identificador de idioma en las traducciones no corresponde con ningún
// idioma soportado por la aplicación.
pub(crate) static FALLBACK_LANGID: LazyLock<LanguageIdentifier> =
    LazyLock::new(|| langid!("en-US"));

// Identificador de idioma **por defecto** para la aplicación.
//
// Se resuelve a partir de [`global::SETTINGS.app.language`](global::SETTINGS). Si el identificador
// de idioma no es válido o no está disponible, se usa [`FALLBACK_LANGID`].
pub(crate) static DEFAULT_LANGID: LazyLock<Option<&LanguageIdentifier>> =
    LazyLock::new(|| LangMatch::resolve(&global::SETTINGS.app.language).as_option());

/// Representa la fuente de idioma (`LanguageIdentifier`) asociada a un recurso.
///
/// Este *trait* permite que distintas estructuras expongan su fuente de idioma de forma uniforme.
pub trait LangId {
    /// Devuelve el identificador de idioma asociado al recurso.
    fn langid(&self) -> &'static LanguageIdentifier;
}

/// Operaciones con los idiomas soportados por PageTop.
///
/// Utiliza [`LangMatch`] para transformar un identificador de idioma en un [`LanguageIdentifier`]
/// soportado por PageTop.
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Coincidencia exacta.
/// let lang = LangMatch::resolve("es-ES");
/// assert_eq!(lang.langid().to_string(), "es-ES");
///
/// // Coincidencia parcial (retrocede al idioma base si no hay variante regional).
/// let lang = LangMatch::resolve("es-EC");
/// assert_eq!(lang.langid().to_string(), "es-ES"); // Porque "es-EC" no está soportado.
///
/// // Idioma no especificado.
/// let lang = LangMatch::resolve("");
/// assert_eq!(lang, LangMatch::Unspecified);
///
/// // Idioma no soportado.
/// let lang = LangMatch::resolve("ja-JP");
/// assert_eq!(lang, LangMatch::Unsupported("ja-JP".to_string()));
/// ```
///
/// Con la siguiente instrucción siempre se obtiene un [`LanguageIdentifier`] válido, ya sea porque
/// resuelve un idioma soportado o porque se aplica el idioma por defecto o, en último caso, el de
/// respaldo ("en-US"):
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Idioma por defecto o de respaldo si no resuelve.
/// let lang = LangMatch::resolve("it-IT");
/// let langid = lang.langid();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LangMatch {
    /// Cuando el identificador de idioma es una cadena vacía.
    Unspecified,
    /// Si encuentra un [`LanguageIdentifier`] en la lista de idiomas soportados por PageTop que
    /// coincide exactamente con el identificador de idioma (p. ej. "es-ES"), o con el identificador
    /// del idioma base (p. ej. "es").
    Found(&'static LanguageIdentifier),
    /// Si el identificador de idioma no está entre los soportados por PageTop.
    Unsupported(String),
}

impl Default for LangMatch {
    /// Resuelve al idioma por defecto y, si no está disponible, al idioma de respaldo ("en-US").
    fn default() -> Self {
        LangMatch::Found(DEFAULT_LANGID.unwrap_or(&FALLBACK_LANGID))
    }
}

impl LangMatch {
    /// Resuelve `language` y devuelve la variante [`LangMatch`] apropiada.
    pub fn resolve(language: impl AsRef<str>) -> Self {
        let language = language.as_ref().trim();

        // Rechaza cadenas vacías.
        if language.is_empty() {
            return Self::Unspecified;
        }

        // Intenta aplicar coincidencia exacta con el código completo (p. ej. "es-MX").
        let lang = language.to_ascii_lowercase();
        if let Some(langid) = LANGUAGES.get(lang.as_str()).map(|(langid, _)| langid) {
            return Self::Found(langid);
        }

        // Si la variante regional no existe, retrocede al idioma base (p. ej. "es").
        if let Some((base_lang, _)) = lang.split_once('-') {
            if let Some(langid) = LANGUAGES.get(base_lang).map(|(langid, _)| langid) {
                return Self::Found(langid);
            }
        }

        // En caso contrario, indica que el idioma no está soportado.
        Self::Unsupported(language.to_string())
    }

    /// Devuelve el [`LanguageIdentifier`] si el idioma fue reconocido.
    ///
    /// Solo retorna `Some` si la variante es [`LangMatch::Found`]. En cualquier otro caso (por
    /// ejemplo, si el identificador es vacío o no está soportado), devuelve `None`.
    ///
    /// Este método es útil cuando se desea acceder directamente al idioma reconocido sin aplicar el
    /// idioma por defecto ni el de respaldo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let lang = LangMatch::resolve("es-ES").as_option();
    /// assert_eq!(lang.unwrap().to_string(), "es-ES");
    ///
    /// let lang = LangMatch::resolve("ja-JP").as_option();
    /// assert!(lang.is_none());
    /// ```
    #[inline]
    pub fn as_option(&self) -> Option<&'static LanguageIdentifier> {
        match self {
            LangMatch::Found(l) => Some(l),
            _ => None,
        }
    }
}

/// Permite a [`LangMatch`] actuar como proveedor de idioma.
///
/// Devuelve el [`LanguageIdentifier`] si la variante es [`LangMatch::Found`]; en caso contrario,
/// devuelve el idioma por defecto de la aplicación y, si tampoco está disponible, el idioma de
/// respaldo ("en-US").
///
/// Resulta útil para usar un valor de [`LangMatch`] como fuente de traducción en [`L10n::lookup()`]
/// o [`L10n::using()`].
impl LangId for LangMatch {
    fn langid(&self) -> &'static LanguageIdentifier {
        match self {
            LangMatch::Found(l) => l,
            _ => DEFAULT_LANGID.unwrap_or(&FALLBACK_LANGID),
        }
    }
}

#[macro_export]
/// Incluye un conjunto de recursos **Fluent** y textos de traducción propios.
macro_rules! include_locales {
    // Se desactiva la inserción de marcas de aislamiento Unicode (FSI/PDI) en los argumentos para
    // mejorar la legibilidad y la compatibilidad en ciertos contextos de renderizado.
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
    Text(Cow<'static, str>),
    Translate(Cow<'static, str>),
}

/// Crea instancias para traducir textos localizados.
///
/// Cada instancia puede representar:
///
/// - Un texto puro (`n()`) que no requiere traducción.
/// - Una clave para traducir un texto de las traducciones predefinidas de PageTop (`l()`).
/// - Una clave para traducir de un conjunto concreto de traducciones (`t()`).
///
/// # Ejemplo
///
/// Los argumentos dinámicos se añaden con `with_arg()` o `with_args()`.
///
/// ```rust
/// # use pagetop::prelude::*;
/// // Texto literal sin traducción.
/// let raw = L10n::n("© 2025 PageTop").get();
///
/// // Traducción simple con clave y argumentos.
/// let hello = L10n::l("greeting")
///     .with_arg("name", "Manuel")
///     .get();
/// ```
///
/// También sirve para traducciones contra un conjunto de recursos concreto.
///
/// ```rust,ignore
/// // Traducción con clave, conjunto de traducciones y fuente de idioma.
/// let bye = L10n::t("goodbye", &LOCALES_CUSTOM).lookup(&LangMatch::resolve("it"));
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
    pub fn n(text: impl Into<Cow<'static, str>>) -> Self {
        L10n {
            op: L10nOp::Text(text.into()),
            ..Default::default()
        }
    }

    /// **l** = *“lookup”*. Crea una instancia para traducir usando una clave del conjunto de
    /// traducciones predefinidas.
    pub fn l(key: impl Into<Cow<'static, str>>) -> Self {
        L10n {
            op: L10nOp::Translate(key.into()),
            ..Default::default()
        }
    }

    /// **t** = *“translate”*. Crea una instancia para traducir usando una clave de un conjunto de
    /// traducciones específico.
    pub fn t(key: impl Into<Cow<'static, str>>, locales: &'static Locales) -> Self {
        L10n {
            op: L10nOp::Translate(key.into()),
            locales,
            ..Default::default()
        }
    }

    /// Añade un argumento `{$arg}` => `value` a la traducción.
    pub fn with_arg(mut self, arg: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.insert(arg.into(), value.into());
        self
    }

    /// Añade varios argumentos a la traducción de una sola vez (p. ej. usando la macro [`hm!`],
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

    /// Resuelve la traducción usando el idioma por defecto o, si no procede, el de respaldo de la
    /// aplicación.
    ///
    /// Devuelve `None` si no aplica o no encuentra una traducción válida.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let text = L10n::l("greeting").with_arg("name", "Manuel").get();
    /// ```
    pub fn get(&self) -> Option<String> {
        self.lookup(&LangMatch::default())
    }

    /// Resuelve la traducción usando la fuente de idioma proporcionada.
    ///
    /// Devuelve `None` si no aplica o no encuentra una traducción válida.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// struct ResourceLang;
    ///
    /// impl LangId for ResourceLang {
    ///     fn langid(&self) -> &'static LanguageIdentifier {
    ///         LangMatch::resolve("es-MX").langid()
    ///     }
    /// }
    ///
    /// let r = ResourceLang;
    /// let text = L10n::l("greeting").with_arg("name", "Usuario").lookup(&r);
    /// ```
    pub fn lookup(&self, language: &impl LangId) -> Option<String> {
        match &self.op {
            L10nOp::None => None,
            L10nOp::Text(text) => Some(text.clone().into_owned()),
            L10nOp::Translate(key) => {
                if self.args.is_empty() {
                    self.locales.try_lookup(language.langid(), key.as_ref())
                } else {
                    self.locales.try_lookup_with_args(
                        language.langid(),
                        key.as_ref(),
                        &self
                            .args
                            .iter()
                            .map(|(k, v)| (Cow::Owned(k.clone()), v.clone().into()))
                            .collect::<HashMap<_, _>>(),
                    )
                }
            }
        }
    }

    /// Traduce el texto y lo devuelve como [`Markup`] usando la fuente de idioma proporcionada.
    ///
    /// Si no se encuentra una traducción válida, devuelve una cadena vacía.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let html = L10n::l("welcome.message").using(&LangMatch::resolve("es"));
    /// ```
    pub fn using(&self, language: &impl LangId) -> Markup {
        PreEscaped(self.lookup(language).unwrap_or_default())
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
