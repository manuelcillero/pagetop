use crate::AutoDefault;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// Formatos disponibles para mostrar las trazas.
///
/// El valor se obtiene de [`global::SETTINGS.log.format`](crate::global::LogSettings::format) y
/// determina la representación textual de los eventos registrados por `tracing`. El valor
/// configurado no distingue entre mayúsculas y minúsculas.
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
        match raw.trim().to_ascii_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "full" => Ok(Self::Full),
            "compact" => Ok(Self::Compact),
            "pretty" => Ok(Self::Pretty),
            _ => Err(D::Error::unknown_variant(
                &raw,
                &["Json", "Full", "Compact", "Pretty"],
            )),
        }
    }
}
