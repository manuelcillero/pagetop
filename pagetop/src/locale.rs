//! Localización (¿i18n ó l10n?).
//!
//! Proporciona soporte a [Fluent](https://www.projectfluent.org/), un conjunto de especificaciones
//! para la localización de aplicaciones, así como implementaciones y buenas prácticas originalmente
//! desarrolladas por Mozilla.
//!
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
//! PageTop utiliza [fluent-templates](https://docs.rs/fluent-templates/) para integrar durante la
//! compilación los recursos de localización en el binario de la aplicación. Básicamente agrupa
//! todos los archivos de los subdirectorios del directorio *src/locales* que tienen un
//! [Identificador de Idioma Unicode](https://docs.rs/unic-langid/) válido y los asigna a su
//! identificador correspondiente:
//!
//! ```text
//! resources/locales
//!              ├── common.ftl
//!              ├── en-US
//!              │   └── main.ftl
//!              ├── es-ES
//!              │   └── main.ftl
//!              ├── es-MX
//!              │   └── main.ftl
//!              └── fr
//!                  └── main.ftl
//! ```
//!
//! Ejemplo de un archivo *src/locales/en-US/main.ftl*:
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
//! Ejemplo de un archivo *src/locales/es-ES/main.ftl*:
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
//! Una vez hayas creado tu directorio de recursos FTL, sólo tienes que usar la poderosa macro
//! [`define_locale!`](crate::define_locale) para integrar fácilmente tus recursos de localización.
//!
//! Luego sólo tendrás que usar la función `t()` para realizar tus traducciones:
//!
//! ```
//! use pagetop::{args, define_locale, t};
//!
//! define_locale!(LOCALE_SAMPLE, "src/locales");
//!
//! fn demo() {
//!     println!("* {}", l("hello-world", Locale::From(&LOCALE_SAMPLE)));
//!     println!("* {}", t("hello-user", Locale::With(&LOCALE_SAMPLE, &args!["userName" => "Julia"])));
//!
//!     let args = args![
//!         "userName" => "Roberto",
//!         "photoCount" => 3,
//!         "userGender" => "male"
//!     ];
//!     println!("* {}\n", t("shared-photos", Locale::With(&LOCALE_SAMPLE, &args)));
//! }
//! ```

use crate::html::{Markup, PreEscaped};
use crate::{args, config, trace, LazyStatic};

pub use fluent_templates;
pub use fluent_templates::fluent_bundle::FluentValue;
pub use fluent_templates::{static_loader as static_locale, Loader, StaticLoader as Locales};

pub use unic_langid::{langid, CharacterDirection, LanguageIdentifier};

use std::collections::HashMap;

static LANGUAGES: LazyStatic<HashMap<String, (LanguageIdentifier, &str)>> = LazyStatic::new(|| {
    args![
        "en"    => (langid!("en-US"), "English"),
        "en-GB" => (langid!("en-GB"), "English (British)"),
        "en-US" => (langid!("en-US"), "English (United States)"),
        "es"    => (langid!("es-ES"), "Spanish"),
        "es-ES" => (langid!("es-ES"), "Spanish (Spain)")
    ]
});

static FALLBACK_LANGID: LazyStatic<LanguageIdentifier> = LazyStatic::new(|| langid!("en-US"));

/// Almacena el Identificador de Idioma Unicode
/// ([Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier))
/// global para la aplicación a partir de `SETTINGS.app.language`.
pub(crate) static DEFAULT_LANGID: LazyStatic<&LanguageIdentifier> =
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
            &*FALLBACK_LANGID
        }
    }
}

pub enum Locale<'a> {
    From(&'a Locales),
    With(&'a Locales, &'a HashMap<String, FluentValue<'a>>),
    Using(
        &'a LanguageIdentifier,
        &'a Locales,
        &'a HashMap<String, FluentValue<'a>>,
    ),
}

pub fn t(key: &str, locale: Locale) -> String {
    translate(key, locale)
}

pub fn e(key: &str, locale: Locale) -> Markup {
    PreEscaped(translate(key, locale))
}

#[inline]
pub(crate) fn translate(key: &str, locale: Locale) -> String {
    match locale {
        Locale::From(locales) => locales.lookup(&DEFAULT_LANGID, key),
        Locale::With(locales, args) => locales.lookup_with_args(&DEFAULT_LANGID, key, args),
        Locale::Using(langid, locales, args) => locales.lookup_with_args(langid, key, args),
    }
    .unwrap_or(key.to_string())
}
