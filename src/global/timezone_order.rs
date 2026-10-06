use crate::AutoDefault;

use serde::{Deserialize, Deserializer};

/// Criterios para ordenar las regiones de zonas horarias que se ofrecen para elegir.
///
/// Se obtiene de [`global::SETTINGS.app.timezone_order`](crate::global::App::timezone_order) y
/// determina el orden de los grupos de [`Timezone::supported_by_region()`]. Es también el orden
/// por defecto de [`form::SelectTimezone`], que puede cambiarse con `with_order()`. En todos los
/// casos el grupo *"Etc"* (`Etc/UTC`) va siempre al final.
///
/// [`Timezone::supported_by_region()`]: crate::datetime::Timezone::supported_by_region
/// [`form::SelectTimezone`]: crate::base::component::form::SelectTimezone
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimezoneOrder {
    /// Primero la región de la zona horaria del sitio y luego el resto, de la más cercana a la más
    /// lejana según su desfase horario. Es el comportamiento por defecto.
    #[default]
    Nearest,
    /// Por nombre de la región. [`form::SelectTimezone`] las ordena por su nombre traducido al
    /// idioma de la página, sin distinguir mayúsculas ni acentos;
    /// `Timezone::supported_by_region()`, que no depende del idioma, por su nombre IANA.
    ///
    /// [`form::SelectTimezone`]: crate::base::component::form::SelectTimezone
    Alphabetical,
    /// En el orden en que se indican las regiones, en
    /// [`global::SETTINGS.app.timezone_regions`](crate::global::App::timezone_regions) o con
    /// `SelectTimezone::with_regions()`; si no se indica ninguna región válida, por nombre, como
    /// `TimezoneOrder::Alphabetical`.
    Listed,
}

impl<'de> Deserialize<'de> for TimezoneOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        let result = match raw.trim().to_ascii_lowercase().as_str() {
            "nearest" => Self::Nearest,
            "alphabetical" => Self::Alphabetical,
            "listed" => Self::Listed,
            _ => {
                let default = Self::default();
                println!(
                    concat!(
                        "\nInvalid value \"{}\" for [app].timezone_order. ",
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
