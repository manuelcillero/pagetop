use crate::util;

use super::{langid, LanguageIdentifier};

use std::collections::HashMap;
use std::sync::LazyLock;

/// Tabla de idiomas soportados por PageTop.
///
/// Cada entrada asocia un código de idioma en minúsculas (por ejemplo, `"en"` o `"es-es"`) con:
///
/// - Su [`LanguageIdentifier`] canónico.
/// - La clave de traducción definida en `src/locale/{lang}/languages.ftl` para mostrar su nombre en
///   el idioma activo.
///
/// Esto permite admitir alias de idioma como `"en"` o `"es"` y, al mismo tiempo, mantener un
/// identificador de idioma canónico (por ejemplo, `langid!("en-US")` o `langid!("es-ES")`).
pub(crate) static LANGUAGES: LazyLock<HashMap<&str, (LanguageIdentifier, &str)>> =
    LazyLock::new(|| {
        util::kv![
            "en"    => ( langid!("en-US"), "english" ),
            "en-gb" => ( langid!("en-GB"), "english_british" ),
            "en-us" => ( langid!("en-US"), "english_united_states" ),
            "es"    => ( langid!("es-ES"), "spanish" ),
            "es-es" => ( langid!("es-ES"), "spanish_spain" ),
        ]
    });
