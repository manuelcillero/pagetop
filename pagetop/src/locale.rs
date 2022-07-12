pub use fluent_templates;
pub use fluent_templates::{Loader as Locale, static_loader as static_locale};
pub use fluent_templates::fluent_bundle::FluentValue;

#[macro_export]
/// Permite integrar fácilmente localización en temas, módulos y componentes.
macro_rules! localize {
    ( $dir_locales:literal $(, $core_locales:literal)? ) => {
        use $crate::locale::*;
        use $crate::app::locale::LANGID;

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
            LOCALES.lookup(&LANGID, key)
        }

        #[allow(dead_code)]
        fn t(
            key: &str,
            args: &std::collections::HashMap<String, FluentValue>
        ) -> String {
            LOCALES.lookup_with_args(&LANGID, key, args)
        }

        #[allow(dead_code)]
        fn e(
            key: &str,
            args: &std::collections::HashMap<String, FluentValue>
        ) -> $crate::html::PreEscaped<String> {
            $crate::html::PreEscaped(
                LOCALES.lookup_with_args(&LANGID, key, args)
            )
        }
    };
}
