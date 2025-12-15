use crate::AutoDefault;

use serde::{Deserialize, Deserializer};

/// Formatos disponibles para mostrar las trazas.
///
/// El valor se obtiene de [`global::SETTINGS.log.format`](crate::global::Log::format) y determina
/// la representación textual de los eventos registrados por `tracing`.
/// El valor configurado no distingue entre mayúsculas y minúsculas.
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogFormat {
    /// Formato JSON estructurado.
    Json,

    /// Formato completo con detalles adicionales. Es el valor por defecto.
    #[default]
    Full,

    /// Formato más conciso y legible.
    Compact,

    /// Formato human-friendly con colores y saltos de línea.
    Pretty,
}

impl<'de> Deserialize<'de> for LogFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        let result = match raw.trim().to_ascii_lowercase().as_str() {
            "json" => Self::Json,
            "full" => Self::Full,
            "compact" => Self::Compact,
            "pretty" => Self::Pretty,
            _ => {
                let default = Self::default();
                println!(
                    concat!(
                        "\nInvalid value \"{}\" for [log].format. ",
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
