//! Localización (L10n).
//!
//! PageTop usa el conjunto de especificaciones [Fluent](https://www.projectfluent.org/) para la
//! localización de aplicaciones.
//!
//! # Sintaxis Fluent (FTL)
//!
//! El formato utilizado para describir los recursos de traducción utilizados por Fluent se llama
//! [FTL](https://www.projectfluent.org/fluent/guide/). FTL está diseñado para ser fácil de leer,
//! pero al mismo tiempo permite representar conceptos complejos del lenguaje natural para tratar
//! género, plurales, conjugaciones, y otros.
//!
//!
//! # Recursos Fluent
//!
//! PageTop usa [fluent-templates](https://docs.rs/fluent-templates/) para integrar los recursos de
//! localización en el binario de la aplicación. El siguiente ejemplo agrupa archivos y subcarpetas
//! de *src/locale* que tienen un [Identificador de Idioma Unicode](https://docs.rs/unic-langid/)
//! válido y los asigna a su identificador correspondiente:
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
//!      └── fr/
//!          ├── default.ftl
//!          └── main.ftl
//! ```
//!
//! Ejemplo de un archivo *src/locale/en-US/main.ftl*:
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
//! Ejemplo del archivo equivalente *src/locale/es-ES/main.ftl*:
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
//! # Cómo aplicar la localización en tu código
//!
//! Una vez hayas creado tu directorio de recursos FTL usa la macro
//! [`use_locale!`](crate::use_locale) para integrarlos en tu módulo o aplicación. Si tus recursos
//! se encuentran en el directorio `"src/locale"` bastará con declarar:
//!
//! ```
//! use pagetop::prelude::*;
//!
//! use_locale!(LOCALE_SAMPLE);
//! ```
//!
//! Y si están en otro directorio, entonces puedes usar:
//!
//! ```
//! use pagetop::prelude::*;
//!
//! use_locale!(LOCALE_SAMPLE in "path/to/locale");
//! ```
//!
//! Usa el componente [L10n](crate::core::component::l10n::L10n) para incluir textos y contenidos
//! opcionalmente traducibles según el contexto de renderizado.

use crate::{config, kv, trace, LazyStatic};

pub use fluent_templates;

pub(crate) use fluent_templates::Loader;
pub(crate) use fluent_templates::StaticLoader as Locales;
pub(crate) use unic_langid::{langid, LanguageIdentifier};

use std::collections::HashMap;

static LANGUAGES: LazyStatic<HashMap<String, (LanguageIdentifier, &str)>> = LazyStatic::new(|| {
    kv![
        "en"    => (langid!("en-US"), "English"),
        "en-GB" => (langid!("en-GB"), "English (British)"),
        "en-US" => (langid!("en-US"), "English (United States)"),
        "es"    => (langid!("es-ES"), "Spanish"),
        "es-ES" => (langid!("es-ES"), "Spanish (Spain)"),
    ]
});

static FALLBACK_LANGID: LazyStatic<LanguageIdentifier> = LazyStatic::new(|| langid!("en-US"));

/// Almacena el Identificador de Idioma Unicode
/// ([Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier))
/// global para la aplicación a partir de `SETTINGS.app.language`.
pub(crate) static LANGID: LazyStatic<&LanguageIdentifier> =
    LazyStatic::new(|| langid_for(config::SETTINGS.app.language.as_str()));

pub fn langid_for(language: &str) -> &LanguageIdentifier {
    match LANGUAGES.get(language) {
        Some((langid, _)) => langid,
        _ => {
            trace::warn!(
                "{} \"{}\"! {}",
                "Failed to set language. Unicode Language Identifier",
                config::SETTINGS.app.language,
                "is not accepted. Using \"en-US\", check the settings file",
            );
            &FALLBACK_LANGID
        }
    }
}
