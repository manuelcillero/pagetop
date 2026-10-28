use pagetop::prelude::*;

use crate::Job;

/// Tipo de función que declara trabajos en un [`CronJobs`].
pub type FnDeclareCronJobs = fn(jobs: &mut CronJobs);

/// Trabajos declarados por las extensiones. Ver [`DeclareCronJobs`].
#[derive(AutoDefault)]
pub struct CronJobs {
    jobs: Vec<Job>,
}

impl CronJobs {
    /// Añade un trabajo.
    pub fn add(&mut self, job: Job) {
        self.jobs.push(job);
    }

    // Recoge los trabajos de todas las extensiones.
    pub(crate) fn collect() -> Vec<Job> {
        let mut jobs = CronJobs::default();
        dispatch_actions(|action: &DeclareCronJobs| (action.f)(&mut jobs));
        jobs.jobs
    }
}

/// Acción para declarar trabajos programados.
///
/// Cada extensión registra esta acción con una función que añade sus trabajos. Se despacha una
/// sola vez, al inicializar `pagetop-cron`, así que los trabajos quedan fijos durante toda la vida
/// de la aplicación, como las propias acciones.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_cron::prelude::*;
///
/// async fn cleanup(_cx: JobContext) -> JobResult {
///     // Borra los carritos abandonados...
///     Ok(())
/// }
///
/// fn declare_jobs(jobs: &mut CronJobs) {
///     jobs.add(
///         Job::new("shop.cleanup", Schedule::cron("30 4 * * *"), cleanup)
///             .with_title(Lc::n("Remove abandoned carts")),
///     );
/// }
///
/// struct Shop;
///
/// #[async_trait]
/// impl Extension for Shop {
///     fn dependencies(&self) -> Vec<ExtensionRef> {
///         vec![&pagetop_cron::Cron]
///     }
///
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![DeclareCronJobs::new(declare_jobs)]
///     }
/// }
/// ```
pub struct DeclareCronJobs {
    f: FnDeclareCronJobs,
    weight: Weight,
}

impl ActionDispatcher for DeclareCronJobs {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareCronJobs {
    /// Registra una función que declara trabajos.
    pub fn new(f: FnDeclareCronJobs) -> Self {
        DeclareCronJobs { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}
