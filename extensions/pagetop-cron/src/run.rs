use pagetop::prelude::*;

use std::fmt;

/// Qué provocó la ejecución de un trabajo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunTrigger {
    /// Su programación, con el planificador de la aplicación (`mode = "internal"`).
    Schedule,
    /// El arranque de la aplicación ([`Job::with_run_on_start()`](crate::Job::with_run_on_start)).
    /// Con `mode = "external"`, en la primera llamada a `POST /cron` que recibe el proceso.
    Startup,
    /// Una llamada a [`run_now()`](crate::run_now).
    Manual,
    /// Su programación, con una llamada externa a `POST /cron` (`mode = "external"`).
    External,
}

impl RunTrigger {
    /// Nombre corto en inglés, para trazas y registros.
    pub const fn as_str(&self) -> &'static str {
        match self {
            RunTrigger::Schedule => "schedule",
            RunTrigger::Startup => "startup",
            RunTrigger::Manual => "manual",
            RunTrigger::External => "external",
        }
    }
}

/// Resultado de una ejecución de un trabajo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobOutcome {
    /// Terminó sin errores.
    Succeeded,
    /// Devolvió un error, con su mensaje y el de sus causas.
    Failed(String),
    /// Superó su tiempo máximo ([`Job::with_timeout()`](crate::Job::with_timeout)) y se canceló.
    TimedOut,
    /// Provocó un `panic!`, con su mensaje. El `panic!` no afecta a la aplicación ni a las
    /// siguientes ejecuciones.
    Panicked(String),
}

impl JobOutcome {
    /// Devuelve `true` si la ejecución terminó sin errores.
    pub fn is_success(&self) -> bool {
        matches!(self, JobOutcome::Succeeded)
    }
}

impl fmt::Display for JobOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobOutcome::Succeeded => f.write_str("succeeded"),
            JobOutcome::Failed(error) => write!(f, "failed: {error}"),
            JobOutcome::TimedOut => f.write_str("timed out"),
            JobOutcome::Panicked(message) => write!(f, "panicked: {message}"),
        }
    }
}

/// Informe de una ejecución terminada de un trabajo.
///
/// Lo devuelve [`run_now()`](crate::run_now), lo recibe la acción [`AfterJobRun`] y se guarda la
/// última de cada trabajo en su [`JobStatus`](crate::JobStatus).
#[derive(Clone, Debug, Getters)]
pub struct JobRun {
    /// Devuelve la clave del trabajo.
    key: CowStr,
    /// Devuelve qué provocó la ejecución.
    #[getters(copy)]
    trigger: RunTrigger,
    /// Devuelve el momento en que empezó, en UTC.
    #[getters(copy)]
    started_at: DateTime<Utc>,
    /// Devuelve cuánto duró.
    #[getters(copy)]
    duration: Duration,
    /// Devuelve cómo terminó.
    outcome: JobOutcome,
}

impl JobRun {
    pub(crate) fn new(
        key: CowStr,
        trigger: RunTrigger,
        started_at: DateTime<Utc>,
        duration: Duration,
        outcome: JobOutcome,
    ) -> Self {
        JobRun {
            key,
            trigger,
            started_at,
            duration,
            outcome,
        }
    }
}

// **< AfterJobRun >********************************************************************************

/// Tipo de función que recibe el informe de cada ejecución terminada. Ver [`AfterJobRun`].
pub type FnAfterJobRun = fn(run: &JobRun);

/// Acción que se despacha al terminar cada ejecución de cualquier trabajo, con su informe.
///
/// Sirve para vigilar los trabajos sin tocarlos: registrar las ejecuciones en una tabla, enviar
/// métricas o avisar cuando uno falla. La función es síncrona y se ejecuta en la tarea del
/// trabajo; para algo lento (p. ej. enviar un correo) lanza una tarea con `tokio::spawn()`. Un
/// `panic!` en la función se registra en las trazas y no afecta al trabajo.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_cron::prelude::*;
///
/// fn alert_on_failure(run: &JobRun) {
///     if !run.outcome().is_success() {
///         trace::warn!("ALERT: job \"{}\" {}", run.key(), run.outcome());
///     }
/// }
///
/// struct Monitor;
///
/// #[async_trait]
/// impl Extension for Monitor {
///     fn dependencies(&self) -> Vec<ExtensionRef> {
///         vec![&pagetop_cron::Cron]
///     }
///
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![AfterJobRun::new(alert_on_failure)]
///     }
/// }
/// ```
pub struct AfterJobRun {
    f: FnAfterJobRun,
    weight: Weight,
}

impl ActionDispatcher for AfterJobRun {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl AfterJobRun {
    /// Registra una función que recibe el informe de cada ejecución.
    pub fn new(f: FnAfterJobRun) -> Self {
        AfterJobRun { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    pub(crate) fn dispatch(run: &JobRun) {
        dispatch_actions(|action: &Self| (action.f)(run));
    }
}
