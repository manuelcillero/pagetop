use crate::AutoDefault;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// Modos de salida y rotación para el registro de trazas.
///
/// El valor se obtiene de [`global::SETTINGS.log.rolling`](crate::global::LogSettings::rolling) y
/// determina si las trazas se muestran por pantalla o se vuelcan en archivos con rotación. El valor
/// configurado no distingue entre mayúsculas y minúsculas.
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogRolling {
    /// Escribe las trazas en la salida estándar (sin rotación de archivos). Es el valor por
    /// defecto.
    #[default]
    Stdout,
    /// Rotación diaria de archivos de traza.
    Daily,
    /// Rotación horaria de archivos de traza.
    Hourly,
    /// Rotación por minutos de archivos de traza.
    Minutely,
    /// Archivo de traza "infinito", sin rotación.
    Endless,
}

impl<'de> Deserialize<'de> for LogRolling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        match raw.trim().to_ascii_lowercase().as_str() {
            "stdout" => Ok(Self::Stdout),
            "daily" => Ok(Self::Daily),
            "hourly" => Ok(Self::Hourly),
            "minutely" => Ok(Self::Minutely),
            "endless" => Ok(Self::Endless),
            _ => Err(D::Error::unknown_variant(
                &raw,
                &["Stdout", "Daily", "Hourly", "Minutely", "Endless"],
            )),
        }
    }
}
