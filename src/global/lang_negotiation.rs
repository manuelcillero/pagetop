use crate::AutoDefault;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// Modos disponibles para negociar el idioma de una petición HTTP.
///
/// El ajuste [`global::SETTINGS.app.lang_negotiation`] determina qué fuentes intervienen en la
/// resolución del idioma efectivo utilizado por [`RequestLocale`] y en la generación de URLs
/// mediante [`Context::route()`].
///
/// [`global::SETTINGS.app.lang_negotiation`]: crate::global::AppSettings::lang_negotiation
/// [`RequestLocale`]: crate::locale::RequestLocale
/// [`Context::route()`]: crate::core::component::Context::route
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum LangNegotiation {
    /// Usa todas las fuentes disponibles para determinar el idioma, en este orden: comprueba el
    /// parámetro `?lang` de la URL; si no está presente o no es válido, usa el idioma preferido del
    /// usuario ([`CurrentUser::language()`]); si no tiene ninguno, usa el idioma configurado en
    /// [`global::SETTINGS.app.language`]; si tampoco está disponible o no es válido, usa la
    /// cabecera HTTP `Accept-Language` o, en su defecto, el idioma de respaldo. Es el
    /// comportamiento por defecto.
    ///
    /// [`CurrentUser::language()`]: crate::auth::CurrentUser::language
    /// [`global::SETTINGS.app.language`]: crate::global::AppSettings::language
    #[default]
    Full,

    /// Igual que `LangNegotiation::Full`, pero sin tener en cuenta el parámetro `?lang` de la URL.
    /// El idioma depende, en este orden, del idioma preferido del usuario, de la configuración, de
    /// la cabecera `Accept-Language` del navegador y, en última instancia, del idioma de respaldo.
    NoQuery,

    /// Usa sólo la configuración o, en su defecto, el idioma de respaldo; ignora el idioma del
    /// usuario, la cabecera `Accept-Language` y el parámetro de la URL. Este modo proporciona un
    /// comportamiento estable con idioma fijo.
    ///
    /// Las extensiones que gestionan usuarios no deben ofrecer elegir idioma en este modo; el
    /// idioma que el usuario tenga guardado se conserva por si se cambia de modo.
    ConfigOnly,
}

impl<'de> Deserialize<'de> for LangNegotiation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        match raw.trim().to_ascii_lowercase().as_str() {
            "full" => Ok(Self::Full),
            "noquery" => Ok(Self::NoQuery),
            "configonly" => Ok(Self::ConfigOnly),
            _ => Err(D::Error::unknown_variant(
                &raw,
                &["Full", "NoQuery", "ConfigOnly"],
            )),
        }
    }
}
