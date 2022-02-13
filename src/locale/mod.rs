use crate::Lazy;
use crate::config::SETTINGS;

use unic_langid::LanguageIdentifier;

pub use fluent_templates::{static_loader as static_locale, Loader as Locale};
pub use fluent_templates;
pub use fluent_templates::fluent_bundle::FluentValue;

/// Almacena el Identificador de Idioma Unicode ([Unicode Language Identifier]
/// (https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier)) de
/// la aplicación, obtenido de `SETTINGS.app.language`.
pub static LANGID: Lazy<LanguageIdentifier> = Lazy::new(|| {
    SETTINGS.app.language.parse().expect("Failed to parse.")
});

#[macro_export]
/// Permite integrar fácilmente localización en tus temas y módulos.
macro_rules! localize {
    ( $DEF_LANGID:literal, $locales:literal $(, $core_locales:literal)? ) => {
        use $crate::locale::*;

        static_locale! {
            static LOCALES = {
                locales: $locales,
                $( core_locales: $core_locales, )?
                fallback_language: $DEF_LANGID,

                // Elimina las marcas Unicode que delimitan los argumentos.
                customise: |bundle| bundle.set_use_isolating(false),
            };
        }

        #[allow(dead_code)]
        pub fn l(key: &str) -> String {
            LOCALES.lookup(&LANGID, key)
        }

        #[allow(dead_code)]
        pub fn t(
            key: &str,
            args: &std::collections::HashMap<String, FluentValue>
        ) -> String {
            LOCALES.lookup_with_args(&LANGID, key, args)
        }
    };
}
