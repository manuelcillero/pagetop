use crate::{global, trace, util};

use super::Tz;

use std::sync::LazyLock;

// Identificador de zona horaria configurado para la aplicación, si es válido.
static CONFIG_TZ: LazyLock<Option<Tz>> = LazyLock::new(|| {
    global::SETTINGS
        .app
        .timezone
        .as_deref()
        .and_then(util::non_blank)
        .and_then(|raw| raw.parse().ok())
});

// Zona horaria de respaldo, garantizada incluso sin configuración válida.
const FALLBACK_TZ: Tz = Tz::UTC;

/// Zona horaria configurada para la aplicación.
///
/// Resuelve [`global::SETTINGS.app.timezone`](crate::global::App::timezone) contra la base IANA de
/// zonas horarias. Si no se ha configurado o el valor no es válido, se aplica la zona horaria de
/// respaldo (`UTC`).
pub struct Timezone;

impl Timezone {
    /// Inicializa la zona horaria por defecto que utilizará la aplicación.
    ///
    /// Debe llamarse durante la inicialización para indicar si la zona horaria por defecto procede
    /// de la configuración, de una configuración no válida o de la zona horaria de respaldo.
    pub(crate) fn init() {
        match global::SETTINGS
            .app
            .timezone
            .as_deref()
            .and_then(util::non_blank)
        {
            Some(raw) => {
                if let Some(tz) = *CONFIG_TZ {
                    trace::debug!("Default timezone \"{tz}\" (from config: \"{raw}\")");
                } else {
                    trace::warn!(
                        "Default timezone \"{FALLBACK_TZ}\" (fallback, invalid config: \"{raw}\")"
                    );
                }
            }
            _ => trace::debug!("Default timezone \"{FALLBACK_TZ}\" (fallback, no config)"),
        }
    }

    /// Devuelve la zona horaria configurada explícitamente, si es válida.
    ///
    /// Si no se ha configurado una zona horaria por defecto o el valor no es válido, devuelve
    /// `None`.
    pub fn try_tz() -> Option<Tz> {
        *CONFIG_TZ
    }

    /// Devuelve siempre la zona horaria de respaldo (`UTC`).
    ///
    /// Es la zona horaria garantizada incluso cuando no haya configuración de la aplicación o
    /// cuando el valor configurado no sea válido.
    pub fn fallback_tz() -> Tz {
        FALLBACK_TZ
    }

    /// Devuelve la zona horaria configurada o, en su defecto, la de respaldo (`UTC`).
    pub fn default_tz() -> Tz {
        Self::try_tz().unwrap_or(FALLBACK_TZ)
    }
}
