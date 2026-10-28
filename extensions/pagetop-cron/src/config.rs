//! Ajustes de configuración de `pagetop-cron`.
//!
//! Ejemplo:
//!
//! ```toml
//! [cron]
//! mode = "internal"
//!
//! # Ajustes propios de un trabajo, por su clave.
//! [cron.jobs."shop.cleanup"]
//! schedule     = "30 4 * * *"
//! timeout_secs = 600
//!
//! [cron.jobs."shop.reminders"]
//! enabled = false
//! ```
//!
//! Uso:
//!
//! ```rust,no_run
//! use pagetop_cron::config::{self, CronMode};
//!
//! assert_eq!(config::SETTINGS.mode, CronMode::Internal);
//! ```
//!
//! Todas las opciones, con su valor por defecto, se documentan en [`CronSettings`] y
//! [`JobSettings`].
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

use serde::Deserialize;

use std::collections::HashMap;

/// Ajustes de la sección `[cron]`, cargados al arrancar la aplicación.
pub static SETTINGS: LazySettings<CronSettings> = LazySettings::new();

/// Sección **`[cron]`** de la configuración.
#[settings(section = "cron")]
pub struct CronSettings {
    /// Cómo se ejecutan los trabajos programados. Por defecto, con el planificador de la propia
    /// aplicación ([`CronMode::Internal`]).
    pub mode: CronMode,
    /// Clave secreta que autoriza las llamadas a `POST /cron` con [`CronMode::External`], enviada
    /// en la cabecera `Authorization: Bearer <clave>`. Obligatoria en ese modo, con 16 caracteres
    /// o más (letras, dígitos, `-` o `_`), o la aplicación no arranca. Conviene guardarla en
    /// `config/local.toml`, que no se versiona.
    pub trigger_token: Option<String>,
    /// Segundos entre dos llamadas a `POST /cron` con [`CronMode::External`]. Cada llamada ejecuta
    /// los trabajos programados en el último intervalo de esta duración, contado en múltiplos
    /// exactos desde el 1 de enero de 1970 (UTC), así que el programa externo debe llamar una vez,
    /// y sólo una, en cada intervalo.
    #[default = 60]
    pub trigger_interval_secs: u64,
    /// Segundos que se espera, al detener la aplicación, a que terminen los trabajos en curso.
    /// Pasado ese tiempo, la aplicación termina igualmente.
    #[default = 30]
    pub shutdown_timeout_secs: u64,

    /// Ajustes de trabajos concretos, cada uno en la subsección `[cron.jobs."clave"]`. Las claves
    /// que no corresponden a ningún trabajo declarado se ignoran con un aviso en las trazas.
    pub jobs: HashMap<String, JobSettings>,
}

/// Forma de ejecutar los trabajos programados. Usado en [`CronSettings::mode`].
#[derive(AutoDefault, Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CronMode {
    /// La propia aplicación ejecuta cada trabajo según su programación mientras está en marcha. Es
    /// el valor por defecto. Con varias instancias de la aplicación, **cada una** ejecuta los
    /// trabajos: deja este modo en una sola y usa [`CronMode::Off`] en las demás.
    #[default]
    Internal,
    /// Los trabajos sólo se ejecutan cuando un programa externo (`crontab`, un temporizador de
    /// *systemd*, un servicio de la nube...) llama a `POST /cron` con la clave
    /// [`trigger_token`](CronSettings::trigger_token), una vez cada
    /// [`trigger_interval_secs`](CronSettings::trigger_interval_secs): cada llamada ejecuta los
    /// trabajos programados desde la anterior. Útil para varias instancias tras un balanceador,
    /// porque cada llamada sólo llega a una de ellas, y para las plataformas que detienen la
    /// aplicación cuando no hay tráfico, porque no depende de lo que recuerde cada proceso.
    External,
    /// Ningún trabajo se ejecuta de forma automática; sólo bajo demanda con
    /// [`run_now()`](crate::run_now). Útil en las instancias que no deben ejecutar trabajos y
    /// para detenerlos temporalmente.
    Off,
}

/// Subsección **`[cron.jobs."clave"]`** de la configuración, con los ajustes de un trabajo.
///
/// Permite a quien administra la aplicación cambiar cuándo se ejecuta un trabajo, o desactivarlo,
/// sin modificar el código. Los ajustes que no se indican mantienen lo declarado en el código.
#[settings]
pub struct JobSettings {
    /// `false` desactiva el trabajo: deja de ejecutarse de forma automática, aunque se puede
    /// ejecutar bajo demanda con [`run_now()`](crate::run_now).
    #[default = true]
    pub enabled: bool,
    /// Sustituye la programación declarada en el código, con la sintaxis de
    /// [`Schedule::parse()`](crate::Schedule::parse) (p. ej. `"*/10 * * * *"` o `"@every 5m"`).
    /// Una programación no válida detiene el arranque.
    pub schedule: Option<String>,
    /// Sustituye el tiempo máximo de cada ejecución, en segundos. `0` quita el límite.
    pub timeout_secs: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::{CronSettings, config};

    // `reference.toml` documents every option; with unknown keys rejected, loading it proves that
    // it only documents keys that exist, with values of the right type.
    const REFERENCE: &str = include_str!("../../../config/reference.toml");

    #[test]
    fn reference_toml_documents_only_existing_keys() {
        config::load_from_str::<CronSettings>(REFERENCE).unwrap();
    }

    // Job keys usually contain dots, which TOML only accepts quoted. They must reach the map as a
    // single key, not as nested tables.
    #[test]
    fn quoted_job_keys_with_dots_are_kept_whole() {
        let settings = config::load_from_str::<CronSettings>(
            r#"
            [cron.jobs."shop.cleanup"]
            enabled = false
            schedule = "@hourly"
            "#,
        )
        .unwrap();
        let job = &settings.jobs["shop.cleanup"];
        assert!(!job.enabled);
        assert_eq!(job.schedule.as_deref(), Some("@hourly"));
        assert_eq!(job.timeout_secs, None);
    }
}
