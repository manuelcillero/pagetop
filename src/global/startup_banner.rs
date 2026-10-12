use crate::AutoDefault;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// Opciones para el *banner* ASCII mostrado al arrancar la aplicación.
///
/// Se obtiene de [`global::SETTINGS.app.startup_banner`] y controla si se muestra un *banner* en la
/// salida estándar al arrancar la aplicación.
///
/// [`global::SETTINGS.app.startup_banner`]: crate::global::AppSettings::startup_banner
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartupBanner {
    /// No muestra ningún banner de inicio.
    Off,
    /// Banner en estilo "Slant". Es el comportamiento por defecto.
    #[default]
    Slant,
    /// Banner en estilo "Small".
    Small,
    /// Banner en estilo "Speed".
    Speed,
    /// Banner en estilo "Starwars".
    Starwars,
}

impl<'de> Deserialize<'de> for StartupBanner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        match raw.trim().to_ascii_lowercase().as_str() {
            "off" => Ok(Self::Off),
            "slant" => Ok(Self::Slant),
            "small" => Ok(Self::Small),
            "speed" => Ok(Self::Speed),
            "starwars" => Ok(Self::Starwars),
            _ => Err(D::Error::unknown_variant(
                &raw,
                &["Off", "Slant", "Small", "Speed", "Starwars"],
            )),
        }
    }
}
