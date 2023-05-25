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

use unic_langid::langid;

pub use fluent_templates;
pub use fluent_templates::fluent_bundle::FluentValue;
pub use fluent_templates::{static_loader as static_locale, Loader, StaticLoader as Locales};

pub use unic_langid::LanguageIdentifier;

use std::collections::HashMap;

static LANGUAGES: LazyStatic<HashMap<String, (LanguageIdentifier, &str)>> = LazyStatic::new(|| {
    args![
        "en"    => (langid!("en-US"), "English"),
        "en-US" => (langid!("en-US"), "English (...)"),
        "es"    => (langid!("es-ES"), "Spanish"),
        "es-ES" => (langid!("es-ES"), "Spanish (Spain)")
    ]
});

static DEFAULT_LANGID: LazyStatic<LanguageIdentifier> = LazyStatic::new(|| langid!("en-US"));

/// Almacena el Identificador de Idioma Unicode
/// ([Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier))
/// para la aplicación, obtenido de `SETTINGS.app.language`.
pub static LANGID: LazyStatic<&LanguageIdentifier> =
    LazyStatic::new(
        || match LANGUAGES.get(config::SETTINGS.app.language.as_str()) {
            Some((langid, _)) => langid,
            _ => {
                trace::warn!(
                    "{}, {} \"{}\"! {}, {}",
                    "Failed to parse language",
                    "unrecognized Unicode Language Identifier",
                    config::SETTINGS.app.language,
                    "Using \"en-US\"",
                    "check the settings file",
                );
                &*DEFAULT_LANGID
            }
        },
    );

#[macro_export]
/// Define un conjunto de elementos de localización y funciones locales de traducción.
macro_rules! define_locale {
    ( $LOCALES:ident, $dir_locales:literal $(, $core_locales:literal)? ) => {
        use $crate::locale::*;

        static_locale! {
            pub static $LOCALES = {
                locales: $dir_locales,
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",

                // Elimina las marcas Unicode que delimitan los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }
    };
}

pub enum Locale<'a> {
    From(&'a Locales),
    With(&'a Locales, &'a HashMap<String, FluentValue<'a>>),
    Lang(&'a Locales, &'a LanguageIdentifier),
    Using(
        &'a Locales,
        &'a LanguageIdentifier,
        &'a HashMap<String, FluentValue<'a>>,
    ),
}

pub fn _t(key: &str, locale: Locale) -> String {
    match locale {
        Locale::From(locales) => locales.lookup(&LANGID, key).unwrap_or(key.to_string()),
        Locale::With(locales, args) => locales
            .lookup_with_args(&LANGID, key, args)
            .unwrap_or(key.to_string()),
        Locale::Lang(locales, langid) => locales.lookup(langid, key).unwrap_or(key.to_string()),
        Locale::Using(locales, langid, args) => locales
            .lookup_with_args(langid, key, args)
            .unwrap_or(key.to_string()),
    }
}

pub fn _e(key: &str, locale: Locale) -> Markup {
    PreEscaped(_t(key, locale))
}
