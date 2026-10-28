use pagetop::prelude::*;

use tokio::sync::watch;

use crate::{RunTrigger, Schedule};

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Resultado de una ejecución de un trabajo.
///
/// Con [`BoxError`] el trabajo puede propagar con `?` cualquier error; el error se registra en las
/// trazas y en el estado del trabajo ([`JobStatus`](crate::JobStatus)).
pub type JobResult = Result<(), BoxError>;

// Función del trabajo, ya preparada para ejecutarse en una tarea propia.
pub(crate) type JobFn =
    Arc<dyn Fn(JobContext) -> Pin<Box<dyn Future<Output = JobResult> + Send>> + Send + Sync>;

/// Qué hacer a la hora de ejecutar un trabajo si no ha terminado la ejecución anterior.
///
/// Usado en [`Job::with_overlap()`]. Las ejecuciones programadas con [`Schedule::fixed_delay()`]
/// nunca se solapan entre sí; con esa programación, sólo decide qué ocurre si se pide
/// [`run_now()`](crate::run_now) durante una de ellas.
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Overlap {
    /// Se omite la nueva ejecución y se registra un aviso. Es el valor por defecto, porque la
    /// mayoría de los trabajos no esperan ejecutarse dos veces a la vez sobre los mismos datos.
    #[default]
    Skip,
    /// Se ejecuta igualmente, a la vez que la anterior.
    Allow,
}

/// Un trabajo programado: una función asíncrona que se ejecuta según una [`Schedule`].
///
/// Se identifica por una clave única, que conviene empezar por el nombre de la extensión
/// (`"shop.cleanup"`), con letras minúsculas, dígitos, `_`, `-` y `.`. La clave aparece en las
/// trazas y es la que usa la configuración para cambiar o desactivar el trabajo
/// (`[cron.jobs."shop.cleanup"]`) y [`run_now()`](crate::run_now) para ejecutarlo bajo demanda.
///
/// La función recibe un [`JobContext`] y devuelve un [`JobResult`]. Accede a los ajustes y a los
/// servicios de la aplicación igual que un *handler*: con los `SETTINGS` de cada extensión, o con
/// funciones como `dbconn()` de `pagetop-seaorm`.
///
/// Los trabajos se declaran con la acción [`DeclareCronJobs`](crate::DeclareCronJobs).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_cron::prelude::*;
///
/// async fn purge_sessions(_cx: JobContext) -> JobResult {
///     // Borra las sesiones caducadas...
///     Ok(())
/// }
///
/// let job = Job::new("shop.purge_sessions", Schedule::cron("0 3 * * *"), purge_sessions)
///     .with_title(Lc::n("Purge expired sessions"))
///     .with_timeout(Duration::minutes(10));
/// ```
#[derive(Clone, Getters)]
pub struct Job {
    /// Devuelve la clave única del trabajo.
    key: CowStr,
    /// Devuelve el título descriptivo del trabajo.
    title: Lc,
    /// Devuelve la programación declarada en el código.
    schedule: Schedule,
    #[getters(skip)]
    run: JobFn,
    /// Devuelve qué hacer si la ejecución anterior aún no ha terminado.
    #[getters(copy)]
    overlap: Overlap,
    /// Devuelve el tiempo máximo de cada ejecución, si lo hay.
    #[getters(copy)]
    timeout: Option<Duration>,
    /// Devuelve la zona horaria propia del trabajo, si la tiene.
    #[getters(copy)]
    timezone: Option<Tz>,
    /// Devuelve si el trabajo se ejecuta también al arrancar la aplicación.
    #[getters(copy)]
    run_on_start: bool,
}

impl Job {
    /// Crea un trabajo con su clave, su programación y la función que ejecuta.
    ///
    /// La función puede ser una `async fn` o un cierre que devuelva un *future*. El *future* debe
    /// ser `Send + 'static`: si el cierre usa datos propios, los clona dentro de un bloque
    /// `async move`.
    pub fn new<F, Fut>(key: impl Into<CowStr>, schedule: Schedule, run: F) -> Self
    where
        F: Fn(JobContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = JobResult> + Send + 'static,
    {
        Job {
            key: key.into(),
            title: Lc::none(),
            schedule,
            run: Arc::new(move |cx| Box::pin(run(cx))),
            overlap: Overlap::default(),
            timeout: None,
            timezone: None,
            run_on_start: false,
        }
    }

    /// Título descriptivo para listados y paneles. Sin título, se muestra la clave.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Qué hacer cuando llega la hora de una ejecución y la anterior aún no ha terminado. Por
    /// defecto, [`Overlap::Skip`].
    pub fn with_overlap(mut self, overlap: Overlap) -> Self {
        self.overlap = overlap;
        self
    }

    /// Tiempo máximo de cada ejecución. Si se supera, la ejecución se cancela (el *future* deja
    /// de ejecutarse en el siguiente `.await`) y se registra como
    /// [`JobOutcome::TimedOut`](crate::JobOutcome::TimedOut). Sin valor, no hay límite.
    ///
    /// Un trabajo que no llega a ningún `.await` (un cálculo largo, una llamada bloqueante) no se
    /// puede cancelar: sigue en marcha hasta terminar y, mientras tanto, cuenta como en curso.
    ///
    /// La configuración puede cambiarlo con `timeout_secs` (ver
    /// [`JobSettings`](crate::config::JobSettings)).
    pub fn with_timeout(mut self, timeout: impl Into<Option<Duration>>) -> Self {
        self.timeout = timeout.into();
        self
    }

    /// Zona horaria en la que se interpreta la expresión cron de este trabajo, p. ej.
    /// `Tz::Europe__Madrid`. Sin valor, la de la aplicación (`[app].timezone`).
    pub fn with_timezone(mut self, timezone: impl Into<Option<Tz>>) -> Self {
        self.timezone = timezone.into();
        self
    }

    /// Ejecuta también el trabajo al arrancar la aplicación, además de según su programación. Útil
    /// para no esperar a la primera ejecución programada, p. ej. para procesar lo que quedó
    /// pendiente al detener la aplicación.
    ///
    /// Con `mode = "external"` se ejecuta en la primera llamada a `POST /cron` que recibe cada
    /// proceso de la aplicación, y con `mode = "off"` no se ejecuta.
    pub fn with_run_on_start(mut self) -> Self {
        self.run_on_start = true;
        self
    }

    // Función del trabajo, para ejecutarla en una tarea propia.
    pub(crate) fn runner(&self) -> JobFn {
        Arc::clone(&self.run)
    }
}

// **< JobContext >*********************************************************************************

/// Datos de la ejecución en curso que recibe la función de un [`Job`].
///
/// Además de indicar qué provocó la ejecución, permite a los trabajos largos saber si la aplicación
/// se está deteniendo, para dejar su trabajo en un punto seguro antes de que se agote el tiempo de
/// espera (`cron.shutdown_timeout_secs`).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_cron::prelude::*;
///
/// async fn send_newsletters(cx: JobContext) -> JobResult {
///     for batch in 0..100 {
///         if cx.is_stopping() {
///             // Lo que falte se enviará en la siguiente ejecución.
///             trace::info!("Stopping \"{}\" at batch {batch}", cx.key());
///             break;
///         }
///         // Envía el lote...
///     }
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct JobContext {
    key: CowStr,
    trigger: RunTrigger,
    scheduled_at: Option<DateTime<Utc>>,
    stop: watch::Receiver<bool>,
}

impl JobContext {
    pub(crate) fn new(
        key: CowStr,
        trigger: RunTrigger,
        scheduled_at: Option<DateTime<Utc>>,
        stop: watch::Receiver<bool>,
    ) -> Self {
        JobContext {
            key,
            trigger,
            scheduled_at,
            stop,
        }
    }

    /// Clave del trabajo.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Qué provocó esta ejecución.
    pub fn trigger(&self) -> RunTrigger {
        self.trigger
    }

    /// Momento en que estaba programada esta ejecución, en UTC. Puede ser algo anterior al momento
    /// real en que empieza. `None` si no se debe a la programación (al arrancar o bajo demanda).
    pub fn scheduled_at(&self) -> Option<DateTime<Utc>> {
        self.scheduled_at
    }

    /// Devuelve `true` si la aplicación se está deteniendo.
    pub fn is_stopping(&self) -> bool {
        *self.stop.borrow()
    }

    /// Espera hasta que la aplicación empiece a detenerse. Pensado para `tokio::select!`, junto a
    /// la espera de otra operación que se quiera interrumpir.
    pub async fn stopping(&self) {
        let mut stop = self.stop.clone();
        if stop.wait_for(|stopping| *stopping).await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}
