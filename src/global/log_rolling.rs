use crate::AutoDefault;

use serde::{Deserialize, Deserializer};

/// Modos de salida y rotación para el registro de trazas.
///
/// El valor se obtiene de [`global::SETTINGS.log.rolling`](crate::global::Log::rolling) y
/// determina si las trazas se muestran por pantalla o se vuelcan en archivos con rotación.
/// El valor configurado no distingue entre mayúsculas y minúsculas.
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogRolling {
    /// Escribe las trazas en la salida estándar (sin rotación de archivos).
    Stdout,
    /// Rotación diaria de archivos de traza.
    #[default]
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
        let result = match raw.trim().to_ascii_lowercase().as_str() {
            "stdout" => Self::Stdout,
            "daily" => Self::Daily,
            "hourly" => Self::Hourly,
            "minutely" => Self::Minutely,
            "endless" => Self::Endless,
            _ => {
                let default = Self::default();
                println!(
                    concat!(
                        "\nInvalid value \"{}\" for [log].rolling. ",
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
