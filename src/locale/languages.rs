use crate::util;

use super::{LanguageIdentifier, langid};

use std::collections::HashMap;
use std::sync::LazyLock;

// Tabla de idiomas soportados por PageTop.
//
// Cada entrada asocia un código de idioma en minúsculas (por ejemplo, "en" o "es-es") con:
//
// - Su `LanguageIdentifier` canónico.
// - La clave de traducción definida en `src/locale/{lang}/languages.ftl` para mostrar su nombre en
//   el idioma activo.
//
// Esto permite admitir alias de idioma como "en" o "es" y, al mismo tiempo, mantener un
// identificador de idioma canónico (por ejemplo, `langid!("en-US")` o `langid!("es-ES")`).
//
// Sólo lo usa `locale::definition`, en el mismo módulo `locale`.
pub(super) static LANGUAGES: LazyLock<HashMap<&str, (LanguageIdentifier, &str)>> =
    LazyLock::new(|| {
        util::kv![
            "en"    => ( langid!("en-US"), "english" ),
            "en-gb" => ( langid!("en-GB"), "english_british" ),
            "en-us" => ( langid!("en-US"), "english_united_states" ),
            "es"    => ( langid!("es-ES"), "spanish" ),
            "es-es" => ( langid!("es-ES"), "spanish_spain" ),
        ]
    });

// Idiomas soportados sin alias: una entrada por identificador canónico (la de `LANGUAGES` cuyo
// código coincide con él, p. ej. "es-es" y no "es"), ordenadas por identificador, con la clave de
// su nombre.
pub(super) static SUPPORTED: LazyLock<Vec<(&'static LanguageIdentifier, &'static str)>> =
    LazyLock::new(|| {
        let mut supported: Vec<_> = LANGUAGES
            .iter()
            .filter(|(code, (langid, _))| langid.to_string().eq_ignore_ascii_case(code))
            .map(|(_, (langid, key))| (langid, *key))
            .collect();
        supported.sort_by_cached_key(|(langid, _)| langid.to_string());
        supported
    });

// Un idioma añadido a `LANGUAGES` sólo con su alias (p. ej. "ca" sin "ca-es") lo aceptaría
// `Locale::resolve()`, pero quedaría fuera de `SUPPORTED` y, con él, del selector de idioma.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_has_its_canonical_entry() {
        for (code, (langid, _)) in LANGUAGES.iter() {
            assert_eq!(
                SUPPORTED
                    .iter()
                    .filter(|(supported, _)| *supported == langid)
                    .count(),
                1,
                "language \"{code}\" has no canonical entry \"{langid}\" in LANGUAGES"
            );
        }
    }
}
