use crate::{Lazy, trace};
use crate::config::SETTINGS;

use unic_langid::LanguageIdentifier;

/// Almacena el Identificador de Idioma Unicode ([Unicode Language Identifier]
/// (https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier)) de
/// la aplicación, obtenido de `SETTINGS.app.language`.
pub static LANGID: Lazy<LanguageIdentifier> = Lazy::new(|| {
    match SETTINGS.app.language.parse() {
        Ok(language) => language,
        Err(_) => {
            trace::warn!(
                "Failed to parse language \"{}\". {}. {}. {}.",
                SETTINGS.app.language,
                "Unicode Language Identifier not recognized",
                "Using \"en-US\"",
                "Check the settings file",
            );
            "en-US".parse().unwrap()
        }
    }
});
