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
//! [`pub_locale!`](crate::pub_locale) para integrar fácilmente tus recursos de localización.
//!
//! Esta macro crea dos funciones para el ámbito donde se ejecuta. Por un lado la función `l()` para
//! traducciones directas de etiquetas. Y por otro la función `t()` para traducciones que requieren
//! argumentos:
//!
//! ```
//! use pagetop::{args, pub_locale};
//!
//! pub_locale!("en-US");
//!
//! fn demo() {
//!     println!("* {}", l("hello-world"));
//!     println!("* {}", t("hello-world", &args![]));
//!     println!("* {}", t("hello-user", &args!["userName" => "Julia"]));
//!
//!     let args = args![
//!         "userName" => "Roberto",
//!         "photoCount" => 3,
//!         "userGender" => "male"
//!     ];
//!     println!("* {}\n", t("shared-photos", &args));
//! }
//! ```

use crate::{config, trace, LazyStatic};

use unic_langid::LanguageIdentifier;

pub use fluent_templates;
pub use fluent_templates::fluent_bundle::FluentValue;
pub use fluent_templates::{static_loader as static_locale, Loader as Locale};

/// Almacena el Identificador de Idioma Unicode
/// ([Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier))
/// para la aplicación, obtenido de `SETTINGS.app.language`.
pub static LANGID: LazyStatic<LanguageIdentifier> =
    LazyStatic::new(|| match config::SETTINGS.app.language.parse() {
        Ok(language) => language,
        Err(_) => {
            trace::warn!(
                "{}, {} \"{}\"! {}, {}",
                "Failed to parse language",
                "unrecognized Unicode Language Identifier",
                config::SETTINGS.app.language,
                "Using \"en-US\"",
                "check the settings file",
            );
            "en-US".parse().unwrap()
        }
    });

#[macro_export]
/// Permite integrar fácilmente localización en temas, módulos y componentes.
macro_rules! pub_locale {
    ( $dir_locales:literal $(, $core_locales:literal)? ) => {
        use $crate::locale::*;

        static_locale! {
            static LOCALES = {
                locales: $dir_locales,
                $( core_locales: $core_locales, )?
                fallback_language: "en-US",

                // Elimina las marcas Unicode que delimitan los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }

        #[allow(dead_code)]
        fn l(key: &str) -> String {
            LOCALES.lookup(&LANGID, key).unwrap_or(key.to_string())
        }

        #[allow(dead_code)]
        fn t(
            key: &str,
            args: &std::collections::HashMap<String, FluentValue>
        ) -> String {
            LOCALES.lookup_with_args(&LANGID, key, args).unwrap_or(key.to_string())
        }

        #[allow(dead_code)]
        fn e(
            key: &str,
            args: &std::collections::HashMap<String, FluentValue>
        ) -> $crate::html::PreEscaped<String> {
            $crate::html::PreEscaped(
                LOCALES.lookup_with_args(&LANGID, key, args).unwrap_or(key.to_string())
            )
        }
    };
}
