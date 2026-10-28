use pagetop::prelude::*;

use tokio::sync::{Notify, watch};

use crate::config::{CronMode, JobSettings, SETTINGS};
use crate::schedule::Trigger;
use crate::{AfterJobRun, CronError, CronJobs, Job, JobContext, JobOutcome, JobRun, JobStatus};
use crate::{Overlap, RunTrigger, Schedule};

use std::any::Any;
use std::collections::HashSet;
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::Instant;

/// Ruta de las llamadas externas que ejecutan los trabajos pendientes.
///
/// `POST /cron` con la cabecera `Authorization: Bearer {trigger_token}`, sólo para
/// `mode = "external"`.
pub const TRIGGER_PATH: &str = "/cron";

// Longitud mínima de `cron.trigger_token`.
const MIN_TOKEN_LEN: usize = 16;

// Espera máxima de una sola vez hasta la próxima ejecución. Al despertar se vuelve a calcular lo
// que falta con el reloj del sistema, así que un cambio de hora del equipo (o una suspensión, en
// la que no avanzan los temporizadores de Tokio) retrasa las ejecuciones como mucho este tiempo.
const MAX_NAP: std::time::Duration = std::time::Duration::from_secs(60);

// Trabajos declarados, construidos una sola vez al inicializar la extensión: preparar de nuevo la
// aplicación en el mismo proceso (las pruebas lo hacen) reutiliza los ya construidos.
static REGISTRY: OnceLock<Registry> = OnceLock::new();

// Evita lanzar dos veces el planificador.
static STARTED: AtomicBool = AtomicBool::new(false);

// Pasa a `true` al detener la aplicación: los bucles del planificador terminan y los trabajos lo
// ven en `JobContext::is_stopping()`.
static STOP: LazyLock<watch::Sender<bool>> = LazyLock::new(|| watch::Sender::new(false));

// Ejecuciones en curso de todos los trabajos, para esperarlas al detener la aplicación.
static ACTIVE: AtomicUsize = AtomicUsize::new(0);
static IDLE: Notify = Notify::const_new();

struct Registry {
    entries: Vec<Arc<Entry>>,
}

// Un trabajo con sus ajustes ya aplicados y su estado.
struct Entry {
    job: Job,
    schedule: Schedule,
    trigger: Trigger,
    timezone: Tz,
    timeout: Option<Duration>,
    enabled: bool,
    running: AtomicUsize,
    state: Mutex<State>,
}

#[derive(AutoDefault)]
struct State {
    next_run: Option<DateTime<Utc>>,
    // Sólo en el modo externo: si falta la ejecución del arranque, y el último intervalo atendido.
    pending_start: bool,
    last_window: Option<DateTime<Utc>>,
    last_run: Option<JobRun>,
    runs: u64,
    failures: u64,
    skipped: u64,
}

// **< Construcción del registro >******************************************************************

// Recoge y valida los trabajos declarados. Un error detiene el arranque de la aplicación.
pub(crate) fn initialize() -> Result<(), CronError> {
    if REGISTRY.get().is_none() {
        let registry = build(CronJobs::collect(), Utc::now())?;
        for entry in &registry.entries {
            trace::debug!(
                "Cron job \"{}\": \"{}\" ({}){}",
                entry.job.key(),
                entry.schedule,
                entry.timezone,
                if entry.enabled { "" } else { ", disabled" }
            );
        }
        let _ = REGISTRY.set(registry);
    }
    Ok(())
}

fn build(jobs: Vec<Job>, now: DateTime<Utc>) -> Result<Registry, CronError> {
    check_external_settings()?;
    let default_tz = Timezone::default_tz();

    let mut keys = HashSet::with_capacity(jobs.len());
    let mut entries = Vec::with_capacity(jobs.len());
    for job in jobs {
        let key = job.key().to_string();
        if !is_valid_key(&key) {
            return Err(CronError::InvalidKey(key));
        }
        let entry = Entry::new(job, SETTINGS.jobs.get(&key), default_tz, now).map_err(|err| {
            CronError::InvalidJob {
                key: key.clone(),
                source: Box::new(err),
            }
        })?;
        if !keys.insert(key.clone()) {
            return Err(CronError::DuplicateJob(key));
        }
        entries.push(Arc::new(entry));
    }
    for key in SETTINGS.jobs.keys() {
        if !keys.contains(key) {
            trace::warn!("Unknown cron job \"{key}\" in [cron.jobs], ignored. Check settings");
        }
    }
    entries.sort_by(|a, b| a.job.key().cmp(b.job.key()));
    Ok(Registry { entries })
}

fn is_valid_key(key: &str) -> bool {
    key.starts_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())
        && key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '_' | '-' | '.'))
}

// El modo externo necesita una clave larga y que se pueda escribir tal cual en la cabecera, y un
// intervalo entre llamadas válido.
fn check_external_settings() -> Result<(), CronError> {
    if SETTINGS.mode != CronMode::External {
        return Ok(());
    }
    let valid = trigger_token().is_some_and(|token| {
        token.len() >= MIN_TOKEN_LEN
            && token
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
    });
    if !valid {
        return Err(CronError::InvalidSetting {
            key: "cron.trigger_token",
            reason: format!(
                "mode \"external\" needs a secret token of at least {MIN_TOKEN_LEN} letters, \
                 digits, '-' or '_'"
            ),
        });
    }
    if trigger_interval().is_none() {
        return Err(CronError::InvalidSetting {
            key: "cron.trigger_interval_secs",
            reason: "must be between 1 second and 100 years".to_owned(),
        });
    }
    Ok(())
}

fn trigger_token() -> Option<&'static str> {
    SETTINGS.trigger_token.as_deref().and_then(util::non_blank)
}

fn trigger_interval() -> Option<Duration> {
    let secs = i64::try_from(SETTINGS.trigger_interval_secs).ok()?;
    Duration::try_seconds(secs)
        .filter(|interval| *interval >= Duration::seconds(1) && *interval <= Duration::days(36_500))
}

// Final del último intervalo completo de llamadas externas, en múltiplos exactos de `interval`
// desde 1970 (UTC): una llamada a las 4:30:20, con intervalos de un minuto, atiende el que va de
// las 4:29:00 a las 4:30:00, y lo mismo haría con cualquier otra instancia de la aplicación.
fn window_end(now: DateTime<Utc>, interval: Duration) -> Option<DateTime<Utc>> {
    let size = interval.num_milliseconds();
    DateTime::from_timestamp_millis(now.timestamp_millis().div_euclid(size).checked_mul(size)?)
}

// Compara la clave recibida con la configurada en un tiempo que no depende de dónde difieren, para
// no dar pistas sobre la clave a quien mida los tiempos de respuesta.
pub(crate) fn token_matches(given: &str) -> bool {
    let Some(expected) = trigger_token() else {
        return false;
    };
    let (given, expected) = (given.as_bytes(), expected.as_bytes());
    given.len() == expected.len()
        && given
            .iter()
            .zip(expected)
            .fold(0, |diff, (a, b)| diff | (a ^ b))
            == 0
}

impl Entry {
    fn new(
        job: Job,
        settings: Option<&JobSettings>,
        default_tz: Tz,
        now: DateTime<Utc>,
    ) -> Result<Self, CronError> {
        let schedule = match settings
            .and_then(|s| s.schedule.as_deref())
            .and_then(util::non_blank)
        {
            Some(text) => Schedule::parse(text)?,
            None => job.schedule().clone(),
        };
        let trigger = schedule.compile()?;
        let timeout = match settings.and_then(|s| s.timeout_secs) {
            Some(0) => None,
            Some(secs) => Some(
                i64::try_from(secs)
                    .ok()
                    .and_then(Duration::try_seconds)
                    .ok_or_else(|| CronError::InvalidSetting {
                        key: "cron.jobs.timeout_secs",
                        reason: "too large".to_owned(),
                    })?,
            ),
            None => job.timeout(),
        };
        if timeout.is_some_and(|timeout| timeout <= Duration::zero()) {
            return Err(CronError::InvalidSetting {
                key: "timeout",
                reason: "must be greater than zero".to_owned(),
            });
        }
        let enabled = settings.is_none_or(|s| s.enabled);
        let timezone = job.timezone().unwrap_or(default_tz);

        // Con el modo externo, la próxima ejecución se conoce desde el arranque; la del arranque,
        // si la hay, la hace la primera llamada.
        let external = enabled && SETTINGS.mode == CronMode::External;
        let pending_start = external && job.run_on_start();
        let next_run = match (external, pending_start) {
            (true, true) => Some(now),
            (true, false) => trigger.aligned_after(now, timezone),
            (false, _) => None,
        };

        Ok(Entry {
            job,
            schedule,
            trigger,
            timezone,
            timeout,
            enabled,
            running: AtomicUsize::new(0),
            state: Mutex::new(State {
                next_run,
                pending_start,
                ..State::default()
            }),
        })
    }

    // Decide si una llamada externa que atiende el intervalo que termina en `end` ejecuta el
    // trabajo, y con qué causa y hora programada. Se decide y se anota en un solo bloqueo, para que
    // dos llamadas simultáneas no lancen la misma ejecución. Un intervalo ya atendido, o anterior
    // (el reloj del sistema retrocedió), no vuelve a ejecutar nada.
    fn take_due(
        &self,
        end: DateTime<Utc>,
        interval: Duration,
    ) -> Option<(RunTrigger, Option<DateTime<Utc>>)> {
        let mut state = self.state();
        let startup = std::mem::take(&mut state.pending_start);
        let mut at = None;
        if state.last_window.is_none_or(|last| last < end) {
            state.last_window = Some(end);
            at = self
                .trigger
                .aligned_after(end - interval, self.timezone)
                .filter(|at| *at <= end);
            state.next_run = self.trigger.aligned_after(end, self.timezone);
        }
        match (at, startup) {
            (Some(at), _) => Some((RunTrigger::External, Some(at))),
            (None, true) => Some((RunTrigger::Startup, None)),
            (None, false) => None,
        }
    }

    // Un `panic!` sólo puede ocurrir fuera de la sección protegida (se ejecutan en otra tarea),
    // así que el estado sigue siendo coherente aunque el `Mutex` quede envenenado.
    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn set_next_run(&self, at: Option<DateTime<Utc>>) {
        self.state().next_run = at;
    }

    // Reserva una ejecución, salvo que el trabajo no admita ejecuciones simultáneas y ya haya una.
    fn try_begin(self: &Arc<Self>) -> Option<RunGuard> {
        let began = match self.job.overlap() {
            Overlap::Skip => self
                .running
                .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok(),
            Overlap::Allow => {
                self.running.fetch_add(1, Ordering::SeqCst);
                true
            }
        };
        if !began {
            return None;
        }
        ACTIVE.fetch_add(1, Ordering::SeqCst);
        Some(RunGuard(Arc::clone(self)))
    }

    fn skip(&self) {
        self.state().skipped += 1;
        trace::warn!(
            "Cron job \"{}\" skipped: the previous run has not finished",
            self.job.key()
        );
    }

    fn record(&self, run: &JobRun) {
        let mut state = self.state();
        state.runs += 1;
        if !run.outcome().is_success() {
            state.failures += 1;
        }
        state.last_run = Some(run.clone());
    }

    fn status(&self) -> JobStatus {
        let state = self.state();
        JobStatus {
            key: self.job.key().clone(),
            title: self.job.title().clone(),
            schedule: self.schedule.clone(),
            timezone: self.timezone,
            enabled: self.enabled,
            running: self.running.load(Ordering::SeqCst),
            next_run: state.next_run,
            last_run: state.last_run.clone(),
            runs: state.runs,
            failures: state.failures,
            skipped: state.skipped,
        }
    }
}

// Ejecución reservada; al soltarla se libera la reserva.
struct RunGuard(Arc<Entry>);

impl Drop for RunGuard {
    fn drop(&mut self) {
        self.0.running.fetch_sub(1, Ordering::SeqCst);
        if ACTIVE.fetch_sub(1, Ordering::SeqCst) == 1 {
            IDLE.notify_waiters();
        }
    }
}

// **< API pública >********************************************************************************

/// Devuelve el estado de todos los trabajos declarados, ordenados por su clave.
///
/// Vacío si la aplicación aún no se ha preparado.
pub fn jobs() -> Vec<JobStatus> {
    REGISTRY
        .get()
        .map(|registry| registry.entries.iter().map(|e| e.status()).collect())
        .unwrap_or_default()
}

/// Devuelve el estado de un trabajo por su clave, o `None` si no existe.
pub fn job(key: impl AsRef<str>) -> Option<JobStatus> {
    find(key.as_ref()).ok().map(|entry| entry.status())
}

/// Ejecuta un trabajo ahora mismo y espera a que termine.
///
/// Funciona con cualquier modo de la sección `[cron]` y aunque la configuración desactive el
/// trabajo, y también en las pruebas, donde el planificador no se pone en marcha. Respeta el tiempo
/// máximo del trabajo, registra la ejecución en su estado y despacha [`AfterJobRun`], como
/// cualquier otra ejecución. No cambia su próxima ejecución programada.
///
/// Devuelve el informe de la ejecución, también si el trabajo falla: el error sólo indica que no se
/// ha podido ejecutar, porque no existe ([`CronError::UnknownJob`]), porque ya se está ejecutando y
/// no admite ejecuciones simultáneas ([`CronError::AlreadyRunning`]) o porque la aplicación aún no
/// se ha preparado ([`CronError::NotInitialized`]).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # use pagetop_cron::prelude::*;
/// # async fn example() -> Result<(), CronError> {
/// let run = pagetop_cron::run_now("shop.cleanup").await?;
/// assert!(run.outcome().is_success());
/// # Ok(())
/// # }
/// ```
pub async fn run_now(key: impl AsRef<str>) -> Result<JobRun, CronError> {
    let key = key.as_ref();
    let entry = find(key)?;
    let guard = entry
        .try_begin()
        .ok_or_else(|| CronError::AlreadyRunning(key.to_owned()))?;
    // En su propia tarea: si se abandona la espera (p. ej. se cierra la conexión de la petición
    // que la pidió), la ejecución sigue con su tiempo máximo y su reserva, y se registra igual.
    match tokio::spawn(execute(guard, RunTrigger::Manual, None)).await {
        Ok(run) => Ok(run),
        Err(err) => std::panic::resume_unwind(err.into_panic()),
    }
}

/// Ejecuta los trabajos programados en el último intervalo de llamadas externas.
///
/// Espera a que terminen y devuelve el informe de cada ejecución. Es lo que hace la ruta
/// `POST /cron` con `mode = "external"`, disponible por si la aplicación quiere ofrecer otra forma
/// de llamarlo (p. ej. una orden de su propia línea de comandos). Sólo funciona en ese modo: en los
/// demás no hace nada y devuelve `0`, porque las ejecuciones las decide el planificador interno o
/// nadie.
///
/// Cada llamada atiende el último intervalo completo de `cron.trigger_interval_secs` (un minuto
/// por defecto), contado en múltiplos exactos desde 1970 (UTC), y ejecuta una vez cada trabajo
/// programado en él; una llamada a las 4:30:20 ejecuta lo programado de las 4:29:00 (excluido) a
/// las 4:30:00. No depende de lo que recuerde cada proceso: con varias instancias tras un
/// balanceador, cada trabajo se ejecuta una sola vez aunque cada llamada llegue a una instancia
/// distinta, y una instancia arrancada por la propia llamada ejecuta igualmente lo que toca. A
/// cambio, el programa externo debe llamar una vez, y sólo una, en cada intervalo: si falta una
/// llamada se pierden sus ejecuciones, y si sobra, dos instancias distintas pueden repetirlas.
///
/// La precisión es la del intervalo: con uno de un minuto, un trabajo de cada diez segundos se
/// ejecuta una vez por minuto. Los trabajos con
/// [`with_run_on_start()`](crate::Job::with_run_on_start) se ejecutan además en la primera llamada
/// que recibe cada proceso.
///
/// Los trabajos se ejecutan a la vez, cada uno en su propia tarea, y la función espera a que
/// terminen todos. Así una plataforma que sólo da CPU mientras se atiende una petición la mantiene
/// hasta el final. Si se abandona la espera (p. ej. el programa externo cierra la conexión), los
/// trabajos siguen igualmente hasta terminar o agotar su tiempo máximo.
pub async fn run_due() -> Vec<JobRun> {
    if SETTINGS.mode != CronMode::External {
        trace::warn!("Cron run_due() ignored: [cron].mode is not \"external\"");
        return Vec::new();
    }
    let (Some(registry), Some(interval)) = (REGISTRY.get(), trigger_interval()) else {
        return Vec::new();
    };
    let Some(end) = window_end(Utc::now(), interval) else {
        return Vec::new();
    };
    let mut tasks = Vec::new();
    for entry in registry.entries.iter().filter(|e| e.enabled) {
        let Some((trigger, at)) = entry.take_due(end, interval) else {
            continue;
        };
        let Some(guard) = entry.try_begin() else {
            entry.skip();
            continue;
        };
        tasks.push(tokio::spawn(execute(guard, trigger, at)));
    }
    let mut runs = Vec::with_capacity(tasks.len());
    for task in tasks {
        // `execute()` ya aísla los `panic!` del trabajo; sólo falla si se detiene el *runtime*.
        if let Ok(run) = task.await {
            runs.push(run);
        }
    }
    runs
}

fn find(key: &str) -> Result<&'static Arc<Entry>, CronError> {
    REGISTRY
        .get()
        .ok_or(CronError::NotInitialized)?
        .entries
        .iter()
        .find(|entry| entry.job.key() == key)
        .ok_or_else(|| CronError::UnknownJob(key.to_owned()))
}

// **< Planificador >*******************************************************************************

// Pone en marcha el planificador según el modo configurado. Sólo una vez por proceso.
pub(crate) fn start() {
    let Some(registry) = REGISTRY.get() else {
        return;
    };
    if STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    let total = registry.entries.len();
    let enabled = registry.entries.iter().filter(|e| e.enabled).count();
    match SETTINGS.mode {
        CronMode::Internal => {
            for entry in registry.entries.iter().filter(|e| e.enabled) {
                tokio::spawn(job_loop(Arc::clone(entry)));
            }
            trace::info!("Cron scheduler started with {enabled} of {total} jobs");
        }
        CronMode::External => trace::info!(
            "Cron jobs ({enabled} of {total}) run on authorized calls to POST {TRIGGER_PATH}"
        ),
        CronMode::Off => {
            trace::info!("Cron jobs ({total}) only run on demand: [cron].mode is \"off\"");
        }
    }
}

// Ejecuta un trabajo según su programación hasta que se detiene la aplicación.
async fn job_loop(entry: Arc<Entry>) {
    let mut stop = STOP.subscribe();
    if entry.job.run_on_start() {
        launch(&entry, RunTrigger::Startup, None).await;
    }
    let mut next = entry.trigger.next_after(Utc::now(), entry.timezone);
    while let Some(at) = next {
        entry.set_next_run(Some(at));
        if !sleep_until(at, &mut stop).await {
            break;
        }
        launch(&entry, RunTrigger::Schedule, Some(at)).await;
        next = entry.trigger.following(at, Utc::now(), entry.timezone);
    }
    if next.is_none() {
        trace::info!(
            "Cron job \"{}\" has no more runs scheduled",
            entry.job.key()
        );
    }
    entry.set_next_run(None);
}

// Lanza una ejecución. Con `fixed_delay()` espera a que termine, porque la siguiente se cuenta
// desde entonces; con el resto, la ejecución sigue en su propia tarea.
async fn launch(entry: &Arc<Entry>, trigger: RunTrigger, at: Option<DateTime<Utc>>) {
    let Some(guard) = entry.try_begin() else {
        entry.skip();
        return;
    };
    if entry.trigger.delay().is_some() {
        execute(guard, trigger, at).await;
    } else {
        tokio::spawn(execute(guard, trigger, at));
    }
}

// Espera hasta `at` según el reloj del sistema. Devuelve `false` si la aplicación se detiene antes.
async fn sleep_until(at: DateTime<Utc>, stop: &mut watch::Receiver<bool>) -> bool {
    loop {
        if *stop.borrow() {
            return false;
        }
        let remaining = match (at - Utc::now()).to_std() {
            Ok(remaining) if !remaining.is_zero() => remaining,
            _ => return true,
        };
        tokio::select! {
            () = tokio::time::sleep(remaining.min(MAX_NAP)) => {}
            changed = stop.changed() => {
                if changed.is_err() {
                    return false;
                }
            }
        }
    }
}

// Ejecuta el trabajo en una tarea propia, que aísla sus `panic!` y permite cancelarlo si supera su
// tiempo máximo, y registra el resultado.
async fn execute(guard: RunGuard, trigger: RunTrigger, at: Option<DateTime<Utc>>) -> JobRun {
    let entry = Arc::clone(&guard.0);
    let key = entry.job.key().clone();
    trace::debug!("Cron job \"{key}\" started ({})", trigger.as_str());

    let cx = JobContext::new(key.clone(), trigger, at, STOP.subscribe());
    let run = entry.job.runner();
    let started_at = Utc::now();
    let clock = Instant::now();
    let mut task = tokio::spawn(async move { run(cx).await });
    let joined = match entry.timeout.and_then(|timeout| timeout.to_std().ok()) {
        Some(limit) => match tokio::time::timeout(limit, &mut task).await {
            Ok(joined) => Some(joined),
            Err(_) => {
                task.abort();
                None
            }
        },
        None => Some((&mut task).await),
    };
    let timed_out = joined.is_none();
    let outcome = match joined {
        None => JobOutcome::TimedOut,
        Some(Ok(Ok(()))) => JobOutcome::Succeeded,
        Some(Ok(Err(err))) => JobOutcome::Failed(error_chain(err.as_ref())),
        Some(Err(err)) if err.is_panic() => JobOutcome::Panicked(panic_message(err.into_panic())),
        Some(Err(_)) => JobOutcome::Failed("cancelled".to_owned()),
    };
    let duration = Duration::from_std(clock.elapsed()).unwrap_or_default();

    let run = JobRun::new(key, trigger, started_at, duration, outcome);
    entry.record(&run);
    match run.outcome() {
        JobOutcome::Succeeded => trace::debug!(
            "Cron job \"{}\" succeeded in {} ms",
            run.key(),
            duration.num_milliseconds()
        ),
        outcome => trace::error!(
            "Cron job \"{}\" {outcome} after {} ms",
            run.key(),
            duration.num_milliseconds()
        ),
    }
    // Un `panic!` en una acción no debe detener el bucle del trabajo ni su reprogramación.
    if let Err(payload) = std::panic::catch_unwind(AssertUnwindSafe(|| AfterJobRun::dispatch(&run)))
    {
        trace::error!(
            "AfterJobRun action panicked for cron job \"{}\": {}",
            run.key(),
            panic_message(payload)
        );
    }
    if timed_out {
        // Cancelar sólo surte efecto en el siguiente `.await` del trabajo: uno que no llega a
        // ninguno (un cálculo largo, una llamada bloqueante) sigue en marcha. Mientras siga, la
        // ejecución cuenta como en curso, para no solaparla con la siguiente ni dejar de esperarla
        // al detener la aplicación.
        tokio::spawn(async move {
            let _ = task.await;
            drop(guard);
        });
    } else {
        drop(guard);
    }
    run
}

// Mensaje del error con el de sus causas. Muchos errores terminan ya con el texto de su causa, así
// que se omite cada causa con la que ya termina el mensaje.
fn error_chain(err: &(dyn std::error::Error + 'static)) -> String {
    let mut message = err.to_string();
    let mut source = err.source();
    while let Some(cause) = source {
        let cause_text = cause.to_string();
        if !message.ends_with(&cause_text) {
            message.push_str(": ");
            message.push_str(&cause_text);
        }
        source = cause.source();
    }
    message
}

// Se desreferencia la caja para que `downcast_ref()` sea el de `dyn Any` y no el de `AnyCast`, que
// se aplicaría a la propia `Box`.
fn panic_message(payload: Box<dyn Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => (*payload).downcast_ref::<&str>().map_or_else(
            || "unknown panic".to_owned(),
            |message| (*message).to_owned(),
        ),
    }
}

// Avisa a los trabajos de que la aplicación se detiene y espera, con un tiempo máximo, a que
// terminen las ejecuciones en curso.
pub(crate) async fn shutdown() {
    STOP.send_replace(true);
    let active = ACTIVE.load(Ordering::SeqCst);
    if active == 0 {
        return;
    }
    let limit = SETTINGS.shutdown_timeout_secs;
    trace::info!("Waiting up to {limit} s for {active} running cron jobs to finish");
    let idle = async {
        loop {
            let notified = IDLE.notified();
            if ACTIVE.load(Ordering::SeqCst) == 0 {
                break;
            }
            notified.await;
        }
    };
    if tokio::time::timeout(std::time::Duration::from_secs(limit), idle)
        .await
        .is_err()
    {
        let running: Vec<String> = jobs()
            .into_iter()
            .filter(|job| job.running() > 0)
            .map(|job| job.key().to_string())
            .collect();
        trace::warn!(
            "Cron jobs still running after {limit} s, stopping anyway: {}",
            running.join(", ")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Settings come from this crate's `config/common.toml` (external mode, Madrid timezone).
    fn load_settings() {
        SETTINGS.load().unwrap();
    }

    async fn noop(_cx: JobContext) -> crate::JobResult {
        Ok(())
    }

    fn job(key: &'static str, schedule: &str) -> Job {
        Job::new(key, Schedule::cron(schedule), noop)
    }

    fn at(text: &str) -> DateTime<Utc> {
        text.parse().unwrap()
    }

    fn build_error(jobs: Vec<Job>) -> CronError {
        load_settings();
        match build(jobs, Utc::now()) {
            Ok(_) => panic!("build should fail"),
            Err(err) => err,
        }
    }

    #[pagetop::test]
    async fn keys_must_be_lowercase_and_simple() {
        for key in ["shop.cleanup", "mail_queue", "a-b.c_d", "0job"] {
            assert!(is_valid_key(key), "{key}");
        }
        for key in [
            "",
            "Shop.cleanup",
            ".hidden",
            "-x",
            "with space",
            "a/b",
            "a:b",
        ] {
            assert!(!is_valid_key(key), "{key}");
        }
        assert!(matches!(
            build_error(vec![job("Bad", "* * * * *")]),
            CronError::InvalidKey(key) if key == "Bad"
        ));
    }

    #[pagetop::test]
    async fn duplicated_keys_stop_the_build() {
        let err = build_error(vec![job("a.job", "* * * * *"), job("a.job", "@daily")]);
        assert!(matches!(err, CronError::DuplicateJob(key) if key == "a.job"));
    }

    #[pagetop::test]
    async fn invalid_schedules_and_timeouts_name_the_job() {
        let err = build_error(vec![job("a.job", "61 * * * *")]);
        assert_eq!(err.to_string(), "invalid job \"a.job\"");
        let CronError::InvalidJob { source, .. } = err else {
            panic!("unexpected error");
        };
        assert!(matches!(*source, CronError::InvalidSchedule { .. }));

        let err = build_error(vec![job("a.job", "@daily").with_timeout(Duration::zero())]);
        assert!(matches!(err, CronError::InvalidJob { .. }));
    }

    #[pagetop::test]
    async fn settings_override_the_declared_job() {
        load_settings();
        let now = at("2026-01-15T10:00:00Z");
        let registry = build(
            vec![
                job("test.overridden", "0 * * * *").with_timeout(Duration::minutes(1)),
                job("test.disabled", "* * * * *"),
                job("test.plain", "0 9 * * *"),
                job("test.startup", "@daily").with_run_on_start(),
            ],
            now,
        )
        .unwrap();
        let entry = |key: &str| {
            registry
                .entries
                .iter()
                .find(|e| e.job.key() == key)
                .unwrap()
        };

        let overridden = entry("test.overridden");
        assert_eq!(overridden.schedule.to_string(), "@delay 1m30s");
        assert_eq!(overridden.timeout, None);
        // External mode: the first run is computed from startup.
        assert_eq!(
            overridden.state().next_run,
            Some(at("2026-01-15T10:01:30Z"))
        );

        let disabled = entry("test.disabled");
        assert!(!disabled.enabled);
        assert_eq!(disabled.state().next_run, None);

        // 9:00 in Madrid is 8:00 UTC in winter: already past, so tomorrow.
        let plain = entry("test.plain");
        assert_eq!(plain.timezone, Tz::Europe__Madrid);
        assert_eq!(plain.state().next_run, Some(at("2026-01-16T08:00:00Z")));

        assert_eq!(entry("test.startup").state().next_run, Some(now));

        // Sorted by key.
        let keys: Vec<_> = registry
            .entries
            .iter()
            .map(|e| e.job.key().to_string())
            .collect();
        assert_eq!(
            keys,
            [
                "test.disabled",
                "test.overridden",
                "test.plain",
                "test.startup"
            ]
        );
    }

    #[pagetop::test]
    async fn external_calls_run_each_scheduled_run_once() {
        load_settings();
        let minute = Duration::minutes(1);
        let registry = build(
            vec![
                job("test.plain", "0 9 * * *"),
                Job::new(
                    "test.rate",
                    Schedule::fixed_rate(Duration::minutes(10)),
                    noop,
                ),
                job("test.startup", "@daily").with_run_on_start(),
            ],
            at("2026-01-15T07:50:00Z"),
        )
        .unwrap();
        let entry = |key: &str| {
            registry
                .entries
                .iter()
                .find(|e| e.job.key() == key)
                .unwrap()
        };

        // A call at 8:00:20 handles the minute that ends at 8:00:00 UTC (9:00 in Madrid).
        let end = window_end(at("2026-01-15T08:00:20.500Z"), minute).unwrap();
        assert_eq!(end, at("2026-01-15T08:00:00Z"));
        let plain = entry("test.plain");
        assert_eq!(
            plain.take_due(end, minute),
            Some((RunTrigger::External, Some(end)))
        );
        // A second call for the same minute, or for an earlier one (clock set back), runs nothing.
        assert_eq!(plain.take_due(end, minute), None);
        assert_eq!(plain.take_due(end - minute, minute), None);
        assert_eq!(plain.take_due(end + minute, minute), None);
        assert_eq!(plain.state().next_run, Some(at("2026-01-16T08:00:00Z")));

        // Intervals count from 1970, so every instance agrees on their runs.
        let rate = entry("test.rate");
        assert_eq!(rate.state().next_run, Some(end));
        assert_eq!(
            rate.take_due(end, minute),
            Some((RunTrigger::External, Some(end)))
        );
        assert_eq!(rate.take_due(end + minute, minute), None);
        let later = end + Duration::minutes(10);
        assert_eq!(
            rate.take_due(later, minute),
            Some((RunTrigger::External, Some(later)))
        );

        // A process started by the call itself still runs what is due.
        let woken = build(
            vec![job("test.plain", "0 9 * * *")],
            at("2026-01-15T08:00:00.300Z"),
        )
        .unwrap();
        assert_eq!(
            woken.entries[0].take_due(end, minute),
            Some((RunTrigger::External, Some(end)))
        );

        // The startup run only happens on the first call.
        let startup = entry("test.startup");
        assert_eq!(
            startup.take_due(end, minute),
            Some((RunTrigger::Startup, None))
        );
        assert_eq!(startup.take_due(end + minute, minute), None);
    }

    #[pagetop::test]
    async fn only_the_configured_token_matches() {
        load_settings();
        assert!(token_matches("test-token-0123456789"));
        assert!(!token_matches("test-token-012345678"));
        assert!(!token_matches("test-token-0123456780"));
        assert!(!token_matches(""));
    }

    #[pagetop::test]
    async fn fixed_rate_keeps_its_pace() {
        let trigger = Schedule::fixed_rate(Duration::seconds(10))
            .compile()
            .unwrap();
        let start = at("2026-01-15T10:00:00Z");
        let tz = Tz::UTC;
        // Woken up a bit late: the next run keeps the pace, it does not drift.
        let late = start + Duration::milliseconds(15);
        assert_eq!(
            trigger.following(start, late, tz),
            Some(at("2026-01-15T10:00:10Z"))
        );
        // Several runs missed (e.g. a suspended machine): they are skipped, keeping the pace.
        let much_later = start + Duration::seconds(35);
        assert_eq!(
            trigger.following(start, much_later, tz),
            Some(at("2026-01-15T10:00:40Z"))
        );
        // Billions of missed runs (1 ms pace, 30 days asleep) keep the pace without overflowing.
        let trigger = Schedule::fixed_rate(Duration::milliseconds(1))
            .compile()
            .unwrap();
        let asleep = start + Duration::days(30) + Duration::microseconds(500);
        assert_eq!(
            trigger.following(start, asleep, tz),
            Some(start + Duration::days(30) + Duration::milliseconds(1))
        );
    }

    #[pagetop::test]
    async fn fixed_delay_counts_from_the_end_of_the_run() {
        let trigger = Schedule::fixed_delay(Duration::seconds(10))
            .compile()
            .unwrap();
        let start = at("2026-01-15T10:00:00Z");
        let finished = start + Duration::seconds(25);
        assert_eq!(
            trigger.following(start, finished, Tz::UTC),
            Some(at("2026-01-15T10:00:35Z"))
        );
    }
}
