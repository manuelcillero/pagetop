use pagetop::prelude::*;

use crate::{JobRun, Schedule};

/// Estado de un trabajo: programación efectiva, próxima ejecución y cómo fue la última.
///
/// Lo devuelven [`jobs()`](crate::jobs) y [`job()`](crate::job), p. ej. para mostrar los trabajos
/// en un panel de administración o comprobarlos en las pruebas. Es una copia: no cambia aunque el
/// trabajo se vuelva a ejecutar.
#[derive(Clone, Debug, Getters)]
pub struct JobStatus {
    /// Devuelve la clave del trabajo.
    pub(crate) key: CowStr,
    /// Devuelve el título del trabajo (`Lc::none()` si no tiene).
    pub(crate) title: Lc,
    /// Devuelve la programación efectiva, la de la configuración si la cambia.
    pub(crate) schedule: Schedule,
    /// Devuelve la zona horaria en la que se interpreta la programación.
    #[getters(copy)]
    pub(crate) timezone: Tz,
    /// Devuelve si el trabajo se ejecuta de forma automática (`false` si lo desactiva la
    /// configuración).
    #[getters(copy)]
    pub(crate) enabled: bool,
    /// Devuelve cuántas ejecuciones del trabajo están en curso.
    #[getters(copy)]
    pub(crate) running: usize,
    /// Devuelve la próxima ejecución programada, en UTC, si la hay.
    #[getters(copy)]
    pub(crate) next_run: Option<DateTime<Utc>>,
    /// Devuelve el informe de la última ejecución terminada, si la hay.
    pub(crate) last_run: Option<JobRun>,
    /// Devuelve cuántas ejecuciones han terminado desde que arrancó la aplicación.
    #[getters(copy)]
    pub(crate) runs: u64,
    /// Devuelve cuántas de esas ejecuciones no terminaron bien (error, tiempo agotado o `panic!`).
    #[getters(copy)]
    pub(crate) failures: u64,
    /// Devuelve cuántas ejecuciones se omitieron porque la anterior aún no había terminado.
    #[getters(copy)]
    pub(crate) skipped: u64,
}
