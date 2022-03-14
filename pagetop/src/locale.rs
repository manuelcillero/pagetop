pub use fluent_templates::{static_loader as static_locale, Loader as Locale};
pub use fluent_templates;
pub use fluent_templates::fluent_bundle::FluentValue;

#[macro_export]
/// Permite integrar fácilmente localización en temas, módulos y componentes.
macro_rules! localize {
    ( $DEF_LANGID:literal, $locales:literal $(, $core_locales:literal)? ) => {
        use $crate::locale::*;
        use $crate::core::server::locale::LANGID;

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
        ) -> crate::core::theme::PreEscaped<String> {
            crate::core::theme::PreEscaped(
                LOCALES.lookup_with_args(&LANGID, key, args)
            )
        }
    };
}
