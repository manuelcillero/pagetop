use pagetop::prelude::*;

use croner::Cron;
use croner::parser::CronParser;

use crate::CronError;

use std::fmt;
use std::str::FromStr;

/// Cuándo se ejecuta un trabajo.
///
/// Hay tres formas de programar un trabajo:
///
/// | Constructor                          | Texto           | Se ejecuta...                       |
/// |--------------------------------------|-----------------|-------------------------------------|
/// | [`cron()`](Self::cron)               | `"*/5 * * * *"` | según una expresión cron.           |
/// | [`fixed_rate()`](Self::fixed_rate)   | `"@every 5m"`   | 5 min tras **empezar** la anterior. |
/// | [`fixed_delay()`](Self::fixed_delay) | `"@delay 5m"`   | 5 min tras **terminar** la anterior.|
///
/// Con `fixed_rate()` las ejecuciones siguen un ritmo fijo, aunque una dure más que el intervalo
/// (lo que ocurra entonces lo decide [`Overlap`](crate::Overlap)). Con `fixed_delay()` nunca se
/// solapan: es la opción natural para procesar colas o pendientes, porque cada ejecución empieza
/// con lo que dejó la anterior.
///
/// Los constructores no validan la programación: lo hace la aplicación al arrancar, que no
/// arranca si alguna no es válida e indica el trabajo afectado. Para validarla antes, por ejemplo
/// si procede de un texto que escribe alguien, usa [`parse()`](Self::parse).
///
/// # Expresiones cron
///
/// Siguen la sintaxis clásica de *cron* de cinco campos, con un sexto campo opcional al principio
/// para los segundos y un séptimo, también opcional, al final para el año:
///
/// ```text
/// ┌────────────── segundo (0-59), opcional
/// │ ┌──────────── minuto (0-59)
/// │ │ ┌────────── hora (0-23)
/// │ │ │ ┌──────── día del mes (1-31)
/// │ │ │ │ ┌────── mes (1-12 o JAN-DEC)
/// │ │ │ │ │ ┌──── día de la semana (0-7 o SUN-SAT; 0 y 7 son domingo)
/// │ │ │ │ │ │ ┌── año (p. ej. 2027), opcional
/// │ │ │ │ │ │ │
/// * * * * * * *
/// ```
///
/// Cada campo admite `*` (cualquier valor), listas (`1,15`), rangos (`1-5`) y pasos (`*/15`,
/// `0-30/10`). Además, el día del mes admite `L` (último día del mes) y `W` (día laborable más
/// cercano: `15W`), y el día de la semana admite `L` (último de ese día en el mes: `5L`, el último
/// viernes) y `#` (enésimo de ese día en el mes: `MON#1`, el primer lunes). Si se indican a la vez
/// el día del mes y el de la semana, basta con que se cumpla uno de ellos, como en *cron*.
///
/// También se aceptan los apodos `@yearly` (o `@annually`), `@monthly`, `@weekly` (el domingo),
/// `@daily` y `@hourly`, que se ejecutan al principio de cada periodo.
///
/// Algunos ejemplos:
///
/// | Expresión          | Se ejecuta...                                          |
/// |--------------------|--------------------------------------------------------|
/// | `*/10 * * * *`     | cada diez minutos.                                     |
/// | `*/30 * * * * *`   | cada treinta segundos.                                 |
/// | `30 4 * * *`       | todos los días a las 4:30.                             |
/// | `0 8-18 * * MON-FRI` | a cada hora en punto, de 8 a 18, de lunes a viernes. |
/// | `0 0 L * *`        | a medianoche del último día de cada mes.               |
/// | `0 9 * * MON#1`    | a las 9:00 del primer lunes de cada mes.               |
///
/// Las expresiones se interpretan en la zona horaria del trabajo
/// ([`Job::with_timezone()`](crate::Job::with_timezone)) o, si no tiene, en la de la aplicación
/// (`[app].timezone`). Con el cambio de hora, una ejecución en una hora que no existe (de 2:00 a
/// 3:00 en primavera, en España) se hace al empezar la hora siguiente, y una en una hora que se
/// repite (de 2:00 a 3:00 en otoño) se hace una sola vez.
///
/// # Intervalos
///
/// En texto, un intervalo es una o varias cantidades con su unidad: `ms` (milisegundos), `s`
/// (segundos), `m` (minutos), `h` (horas) y `d` (días); p. ej. `"30s"`, `"5m"` o `"1h30m"`. No se
/// interpretan en ninguna zona horaria: un día es siempre de 24 horas.
///
/// La primera ejecución llega pasado un intervalo desde que arranca la aplicación, salvo que el
/// trabajo se ejecute también al arrancar ([`Job::with_run_on_start()`]).
///
/// Con el modo externo (`mode = "external"`) no se sabe cuándo empezó o terminó la ejecución
/// anterior, que pudo hacerla otra instancia de la aplicación. Ahí los dos intervalos se cuentan
/// igual, en múltiplos exactos desde el 1 de enero de 1970 (UTC): `"@every 10m"` se ejecuta a las
/// 10:00, 10:10, 10:20... (en UTC), y `"@delay 10m"` también.
///
/// [`Job::with_run_on_start()`]: crate::Job::with_run_on_start
///
/// # Ejemplo
///
/// Los intervalos son el [`Duration`] de las fechas de PageTop, que ya incluye su preludio.
///
/// ```rust
/// # use pagetop::prelude::*;
/// # use pagetop_cron::prelude::*;
/// let nightly = Schedule::cron("30 4 * * *");
/// let queue = Schedule::fixed_delay(Duration::seconds(10));
///
/// // Desde texto, p. ej. de la configuración, con validación.
/// assert_eq!(Schedule::parse("@delay 10s").unwrap(), queue);
/// assert!(Schedule::parse("61 * * * *").is_err());
/// assert_eq!(nightly.to_string(), "30 4 * * *");
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schedule(Kind);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Kind {
    Cron(String),
    FixedRate(Duration),
    FixedDelay(Duration),
}

impl Schedule {
    /// Programa el trabajo con una expresión cron (ver [la sintaxis](#expresiones-cron)).
    pub fn cron(expression: impl Into<String>) -> Self {
        Schedule(Kind::Cron(expression.into()))
    }

    /// Programa el trabajo cada `period`, contado desde que **empieza** la ejecución anterior.
    pub fn fixed_rate(period: Duration) -> Self {
        Schedule(Kind::FixedRate(period))
    }

    /// Programa el trabajo `delay` después de que **termina** la ejecución anterior.
    pub fn fixed_delay(delay: Duration) -> Self {
        Schedule(Kind::FixedDelay(delay))
    }

    /// Lee y valida una programación escrita como texto.
    ///
    /// Admite una expresión cron o un apodo como `@daily` (ver [la sintaxis](#expresiones-cron)),
    /// `"@every <intervalo>"` para [`fixed_rate()`](Self::fixed_rate) y `"@delay <intervalo>"`
    /// para [`fixed_delay()`](Self::fixed_delay) (ver [los intervalos](#intervalos)). Es la
    /// sintaxis de la clave `schedule` de los ajustes de cada trabajo, y la misma que devuelve
    /// [`to_string()`](ToString::to_string).
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// # use pagetop_cron::prelude::*;
    /// let schedule = Schedule::parse("@every 1h30m").unwrap();
    /// assert_eq!(schedule, Schedule::fixed_rate(Duration::minutes(90)));
    /// assert_eq!(schedule.to_string(), "@every 1h30m");
    /// ```
    pub fn parse(text: &str) -> Result<Self, CronError> {
        let text = text.trim();
        let (head, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
        let schedule = if head.eq_ignore_ascii_case("@every") {
            Schedule::fixed_rate(parse_duration(rest).map_err(|reason| invalid(text, reason))?)
        } else if head.eq_ignore_ascii_case("@delay") {
            Schedule::fixed_delay(parse_duration(rest).map_err(|reason| invalid(text, reason))?)
        } else {
            Schedule::cron(text)
        };
        schedule.compile()?;
        Ok(schedule)
    }

    /// Devuelve la primera ejecución posterior a `after` según esta programación, interpretando
    /// las expresiones cron en la zona horaria `tz`.
    ///
    /// Para [`fixed_rate()`](Self::fixed_rate) y [`fixed_delay()`](Self::fixed_delay) devuelve
    /// `after` más el intervalo, suponiendo que `after` es el momento en que empieza (o termina)
    /// la ejecución anterior. Devuelve `None` si una expresión cron ya no se cumple nunca (p. ej.
    /// la de un año ya pasado), y un error si la programación no es válida.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// # use pagetop_cron::prelude::*;
    /// let after = "2026-01-15T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
    /// let next = Schedule::cron("0 9 * * *").next_after(after, Tz::Europe__Madrid).unwrap();
    /// // Las 9:00 en Madrid son las 8:00 UTC en invierno.
    /// assert_eq!(next.unwrap().to_rfc3339(), "2026-01-15T08:00:00+00:00");
    /// ```
    pub fn next_after(
        &self,
        after: DateTime<Utc>,
        tz: Tz,
    ) -> Result<Option<DateTime<Utc>>, CronError> {
        Ok(self.compile()?.next_after(after, tz))
    }

    // Valida la programación y la prepara para calcular sus ejecuciones.
    pub(crate) fn compile(&self) -> Result<Trigger, CronError> {
        match &self.0 {
            Kind::Cron(expression) => CronParser::new()
                .parse(expression)
                .map(|cron| Trigger::Cron(Box::new(cron)))
                .map_err(|err| invalid(expression, err.to_string())),
            Kind::FixedRate(period) => Ok(Trigger::FixedRate(positive(self, *period)?)),
            Kind::FixedDelay(delay) => Ok(Trigger::FixedDelay(positive(self, *delay)?)),
        }
    }
}

impl FromStr for Schedule {
    type Err = CronError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Schedule::parse(text)
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Kind::Cron(expression) => f.write_str(expression),
            Kind::FixedRate(period) => write!(f, "@every {}", DurationText(*period)),
            Kind::FixedDelay(delay) => write!(f, "@delay {}", DurationText(*delay)),
        }
    }
}

// **< Trigger >************************************************************************************

// Programación ya validada, lista para calcular las ejecuciones.
pub(crate) enum Trigger {
    Cron(Box<Cron>),
    FixedRate(Duration),
    FixedDelay(Duration),
}

impl Trigger {
    // Primera ejecución posterior a `after`.
    pub(crate) fn next_after(&self, after: DateTime<Utc>, tz: Tz) -> Option<DateTime<Utc>> {
        match self {
            Trigger::Cron(cron) => cron
                .find_next_occurrence(&after.with_timezone(&tz), false)
                .ok()
                .map(|next| next.with_timezone(&Utc)),
            Trigger::FixedRate(interval) | Trigger::FixedDelay(interval) => {
                after.checked_add_signed(*interval)
            }
        }
    }

    // Primera ejecución posterior a `after` sin depender de la anterior, para el modo externo: los
    // intervalos se cuentan en múltiplos exactos desde 1970 (UTC), así que todas las instancias de
    // la aplicación, y cualquier proceso recién arrancado, obtienen las mismas ejecuciones.
    pub(crate) fn aligned_after(&self, after: DateTime<Utc>, tz: Tz) -> Option<DateTime<Utc>> {
        match self {
            Trigger::Cron(_) => self.next_after(after, tz),
            Trigger::FixedRate(interval) | Trigger::FixedDelay(interval) => {
                let period = interval.num_milliseconds();
                let next = after.timestamp_millis().div_euclid(period).checked_add(1)?;
                DateTime::from_timestamp_millis(next.checked_mul(period)?)
            }
        }
    }

    // Ejecución que sigue a la programada en `at`, ya pasada, cuando el reloj marca `now`.
    //
    // Con `fixed_rate()` se suma el intervalo a la ejecución programada, no al momento en que
    // se despertó el planificador, para que el ritmo no se retrase un poco en cada ejecución. Si
    // la siguiente también ha pasado (el equipo estuvo suspendido o el reloj saltó hacia
    // delante), se omiten las perdidas y se mantiene el ritmo.
    pub(crate) fn following(
        &self,
        at: DateTime<Utc>,
        now: DateTime<Utc>,
        tz: Tz,
    ) -> Option<DateTime<Utc>> {
        match self {
            Trigger::FixedRate(period) => {
                let next = at.checked_add_signed(*period)?;
                if next > now {
                    return Some(next);
                }
                let period = period.num_milliseconds();
                let missed = (now - at).num_milliseconds() / period + 1;
                at.checked_add_signed(Duration::try_milliseconds(missed.checked_mul(period)?)?)
            }
            _ => self.next_after(at.max(now), tz),
        }
    }

    // Intervalo de espera tras cada ejecución, sólo con `fixed_delay()`.
    pub(crate) fn delay(&self) -> Option<Duration> {
        match self {
            Trigger::FixedDelay(delay) => Some(*delay),
            _ => None,
        }
    }
}

// **< Funciones auxiliares >***********************************************************************

fn invalid(schedule: &str, reason: impl Into<String>) -> CronError {
    CronError::InvalidSchedule {
        schedule: schedule.to_owned(),
        reason: reason.into(),
    }
}

// El intervalo debe ser de 1 ms a 100 años.
fn positive(schedule: &Schedule, interval: Duration) -> Result<Duration, CronError> {
    if interval < Duration::milliseconds(1) || interval > Duration::days(36_500) {
        return Err(invalid(
            &schedule.to_string(),
            "the interval must be between 1 ms and 100 years",
        ));
    }
    Ok(interval)
}

// Lee un intervalo como "30s", "5m" o "1h 30m".
fn parse_duration(text: &str) -> Result<Duration, String> {
    const TOO_LARGE: &str = "interval too large";
    let text = text.trim();
    if text.is_empty() {
        return Err("missing interval, e.g. \"5m\"".to_owned());
    }
    let mut total: i64 = 0;
    let mut rest = text;
    while !rest.is_empty() {
        let digits = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        if digits == 0 {
            return Err(format!("expected a number in \"{rest}\""));
        }
        let amount: i64 = rest[..digits].parse().map_err(|_| TOO_LARGE.to_owned())?;
        rest = &rest[digits..];
        let unit = rest
            .find(|c: char| !c.is_ascii_alphabetic())
            .unwrap_or(rest.len());
        let millis = match &rest[..unit] {
            "ms" => 1,
            "s" => 1_000,
            "m" => 60_000,
            "h" => 3_600_000,
            "d" => 86_400_000,
            "" => return Err(format!("missing unit after {amount} (ms, s, m, h or d)")),
            other => return Err(format!("unknown unit \"{other}\" (ms, s, m, h or d)")),
        };
        total = amount
            .checked_mul(millis)
            .and_then(|part| total.checked_add(part))
            .ok_or_else(|| TOO_LARGE.to_owned())?;
        rest = rest[unit..].trim_start();
    }
    Duration::try_milliseconds(total).ok_or_else(|| TOO_LARGE.to_owned())
}

// Escribe un intervalo con la sintaxis de `parse_duration()`, p. ej. "1h30m".
struct DurationText(Duration);

impl fmt::Display for DurationText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut millis = self.0.num_milliseconds();
        if millis == 0 {
            return f.write_str("0s");
        }
        if millis < 0 {
            f.write_str("-")?;
            millis = millis.saturating_abs();
        }
        for (unit, size) in [
            ("d", 86_400_000),
            ("h", 3_600_000),
            ("m", 60_000),
            ("s", 1_000),
            ("ms", 1),
        ] {
            if millis >= size {
                write!(f, "{}{unit}", millis / size)?;
                millis %= size;
            }
        }
        Ok(())
    }
}
