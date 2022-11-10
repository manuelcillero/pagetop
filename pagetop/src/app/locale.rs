use crate::{global, trace, LazyStatic};

use unic_langid::LanguageIdentifier;

/// Almacena el Identificador de Idioma Unicode
/// ([Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier))
/// de la aplicación, obtenido de `SETTINGS.app.language`.
pub static LANGID: LazyStatic<LanguageIdentifier> =
    LazyStatic::new(|| match global::SETTINGS.app.language.parse() {
        Ok(language) => language,
        Err(_) => {
            trace::warn!(
                "{}, {} \"{}\"! {}, {}",
                "Failed to parse language",
                "unrecognized Unicode Language Identifier",
                global::SETTINGS.app.language,
                "Using \"en-US\"",
                "check the settings file",
            );
            "en-US".parse().unwrap()
        }
    });
