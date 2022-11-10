use crate::{config, trace, LazyStatic};

use unic_langid::LanguageIdentifier;

// LOCALIZACIÓN ************************************************************************************

/// Almacena el Identificador de Idioma Unicode
/// ([Unicode Language Identifier](https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier))
/// global para la aplicación, obtenido de `SETTINGS.app.language`.
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
