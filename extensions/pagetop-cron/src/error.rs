//! Errores de `pagetop-cron`.

/// Errores al declarar, configurar o ejecutar trabajos programados.
#[derive(Debug, thiserror::Error)]
pub enum CronError {
    /// Una programación no es válida (p. ej. una expresión cron mal escrita o un intervalo nulo).
    #[error("invalid schedule \"{schedule}\": {reason}")]
    InvalidSchedule {
        /// Programación tal como se indicó.
        schedule: String,
        /// Motivo del error.
        reason: String,
    },
    /// La clave de un trabajo no es válida.
    #[error(
        "invalid job key \"{0}\": use lowercase letters, digits, '_', '-' and '.', starting with a \
         letter or digit"
    )]
    InvalidKey(String),
    /// Dos trabajos se declaran con la misma clave.
    #[error("job \"{0}\" is declared more than once")]
    DuplicateJob(String),
    /// Un trabajo no se puede programar, por su declaración o por sus ajustes.
    #[error("invalid job \"{key}\"")]
    InvalidJob {
        /// Clave del trabajo.
        key: String,
        /// Motivo del error.
        #[source]
        source: Box<CronError>,
    },
    /// Un ajuste de la sección `[cron]` no es válido.
    #[error("invalid cron setting \"{key}\": {reason}")]
    InvalidSetting {
        /// Clave del ajuste, p. ej. `"cron.trigger_token"`.
        key: &'static str,
        /// Motivo del error.
        reason: String,
    },
    /// No hay ningún trabajo declarado con la clave indicada.
    #[error("unknown job \"{0}\"")]
    UnknownJob(String),
    /// El trabajo ya se está ejecutando y no admite ejecuciones simultáneas
    /// ([`Overlap::Skip`](crate::Overlap::Skip)).
    #[error("job \"{0}\" is already running")]
    AlreadyRunning(String),
    /// Los trabajos aún no están disponibles: la aplicación no se ha preparado.
    #[error("cron jobs are not available until the application is prepared")]
    NotInitialized,
}
