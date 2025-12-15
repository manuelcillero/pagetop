use crate::AutoDefault;

use serde::{Deserialize, Deserializer};

/// Opciones para el *banner* ASCII mostrado al arrancar la aplicación.
///
/// Se obtiene de [`global::SETTINGS.app.startup_banner`](crate::global::App::startup_banner) y
/// controla si se muestra un *banner* en la salida estándar al arrancar la aplicación.
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
        let result = match raw.trim().to_ascii_lowercase().as_str() {
            "off" => Self::Off,
            "slant" => Self::Slant,
            "small" => Self::Small,
            "speed" => Self::Speed,
            "starwars" => Self::Starwars,
            _ => {
                let default = Self::default();
                println!(
                    concat!(
                        "\nInvalid value \"{}\" for [app].startup_banner. ",
                        "Using \"{:?}\". Check settings.",
                    ),
                    raw, default,
                );
                default
            }
        };
        Ok(result)
    }
}
