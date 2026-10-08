//! Soporte a fechas y horas según estándar [ISO 8601].
//!
//! Basado en [chrono] y zonas horarias de [chrono-tz] con formatos de visualización según idioma.
//!
//! Las fechas y horas se graban siempre en UTC y se muestran en la zona horaria efectiva del
//! usuario actual ([`CurrentUser::timezone()`]), ya sea la suya propia si tiene una configurada y
//! válida y [`timezone_per_user`] lo permite; y si no, la configurada para la aplicación
//! ([`Timezone`]); o, en su defecto, UTC. La conversión sólo ocurre al mostrar
//! ([`Contextual::format_datetime()`]), nunca al guardar.
//!
//! Las reglas de cada zona horaria (desfase respecto a UTC, horario de verano) son las de la base
//! de datos de zonas horarias de la IANA que incluye [chrono-tz] al compilar. Como los países
//! cambian sus reglas, una aplicación compilada sigue usando las reglas anteriores hasta que se
//! recompila con una versión más reciente de `chrono-tz`; conviene hacerlo periódicamente. La traza
//! de arranque indica la versión de la base de datos en uso (p. ej. `2025b`).
//!
//! [`DateFormat`] (fecha) y [`TimeFormat`] (hora) son tipos independientes: no existe un formato
//! combinado de fecha y hora. Para mostrar ambas, [`Contextual::format_datetime()`] junta el
//! resultado de cada uno (posiblemente con formatos distintos, p. ej. fecha larga con hora corta)
//! mediante la clave Fluent `datetime_join` del idioma efectivo. El orden día/mes/año, el nombre
//! del mes y el separador de fecha y hora son una propiedad del idioma, no de la configuración de
//! la aplicación (a diferencia de la zona horaria). Se resuelven como claves Fluent normales
//! (`src/locale/{lang}/datetime.ftl`), con el mismo mecanismo que cualquier otro texto traducido de
//! PageTop.
//!
//! Las fechas y horas que escribe el usuario se leen en el formato del idioma efectivo (p. ej.
//! `dd/mm/aaaa` en español) y, si son instantes, en su hora local, para grabarlas en UTC. Los
//! campos de [`form::date::Field`] las muestran así, y [`Contextual::parse_date()`],
//! [`Contextual::parse_time()`] y [`Contextual::parse_datetime()`] las leen y devuelven un
//! [`DateInputError`] si no se pueden interpretar.
//!
//! [`RelativeFormat`] muestra una fecha en relación al momento actual ("hace 3 años", "dentro de 5
//! días") en vez de como fecha absoluta, con el mismo mecanismo de claves Fluent, vía
//! [`Contextual::format_relative()`].
//!
//! [`DatePrecision`] muestra una fecha de inicio o fin con precisión reducida ("desde junio",
//! "hasta junio de 2026"), vía [`Contextual::format_since()`] y [`Contextual::format_until()`].
//!
//! [ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601
//! [chrono]: https://docs.rs/chrono
//! [chrono-tz]: https://docs.rs/chrono-tz
//! [`timezone_per_user`]: crate::global::App::timezone_per_user
//! [`CurrentUser::timezone()`]: crate::auth::CurrentUser::timezone
//! [`form::date::Field`]: crate::base::component::form::date::Field
//! [`Contextual::format_datetime()`]: crate::core::component::Contextual::format_datetime
//! [`Contextual::format_relative()`]: crate::core::component::Contextual::format_relative
//! [`Contextual::format_since()`]: crate::core::component::Contextual::format_since
//! [`Contextual::format_until()`]: crate::core::component::Contextual::format_until
//! [`Contextual::parse_date()`]: crate::core::component::Contextual::parse_date
//! [`Contextual::parse_time()`]: crate::core::component::Contextual::parse_time
//! [`Contextual::parse_datetime()`]: crate::core::component::Contextual::parse_datetime

use crate::locale::Lc;

use thiserror::Error;

// Reexportado para que el resto del código y las extensiones se refieran siempre a `datetime::X`,
// nunca a `chrono` directamente (mismo criterio que `Tz`/`TZ_VARIANTS` más abajo). No es todo el
// `chrono::prelude` porque se excluyen `Local` (incompatible con el invariante "siempre UTC" de
// este módulo), `Month` (su nombre de mes es inglés fijo, contradice el mecanismo Fluent de este
// módulo) y `SubsecRound`/`SecondsFormat`/`Offset` (sin ningún consumidor, ni directo ni
// indirecto). `Weekday` y `FixedOffset` se mantienen pese a no tener tampoco uso propio: `Weekday`
// es el tipo que ya devuelve `Datelike::weekday()` (en uso interno); `FixedOffset` es el tipo que
// devolvería cualquier futuro *parsing* de fecha ISO (`DateTime::parse_from_rfc3339()`).
pub use chrono::DateTime;
pub use chrono::TimeZone;
pub use chrono::{Datelike, Timelike, Weekday};
pub use chrono::{FixedOffset, Utc};
pub use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

// `Duration` y `Months` no forman parte de `chrono::prelude`, pero son de uso habitual junto al
// resto de tipos de este módulo (sumar/restar intervalos a un `NaiveDateTime`, calcular
// expiraciones, desglosar una diferencia de calendario en años/meses, etc.) que se reexportan
// igualmente.
pub use chrono::{Duration, Months};

// Reexportado para que el resto del código y las extensiones se refieran siempre a `datetime::Tz`/
// `datetime::TZ_VARIANTS`, sin depender directamente de `chrono_tz` (mismo criterio que
// `locale::LanguageIdentifier` sobre `unic_langid`).
pub use chrono_tz::{TZ_VARIANTS, Tz};

mod definition;
pub use definition::Timezone;

mod region;
pub use region::TzRegion;

mod format;
pub use format::{DateFormat, TimeFormat};

mod relative;
pub use relative::RelativeFormat;

mod precision;
pub use precision::DatePrecision;

pub(crate) mod input;

/// Errores al leer una fecha u hora escrita por el usuario.
///
/// Los devuelven [`Contextual::parse_date()`], [`Contextual::parse_time()`] y
/// [`Contextual::parse_datetime()`]. [`message()`](Self::message) da un mensaje traducido para
/// mostrarlo junto al formulario.
///
/// [`Contextual::parse_date()`]: crate::core::component::Contextual::parse_date
/// [`Contextual::parse_time()`]: crate::core::component::Contextual::parse_time
/// [`Contextual::parse_datetime()`]: crate::core::component::Contextual::parse_datetime
#[derive(Clone, Debug, Error, PartialEq)]
pub enum DateInputError {
    /// La fecha no está escrita en el formato del idioma (`hint`, p. ej. `dd/mm/aaaa`) ni en el
    /// formato ISO 8601, o no es una fecha válida (p. ej. `31/02/2026`).
    #[error("invalid date, expected format {hint}")]
    InvalidDate {
        /// Indicación del formato esperado, en el idioma del usuario.
        hint: String,
    },
    /// La hora no está escrita en el formato del idioma (`hint`, p. ej. `hh:mm`) ni en el formato
    /// ISO 8601, o no es una hora válida (p. ej. `25:00`).
    #[error("invalid time, expected format {hint}")]
    InvalidTime {
        /// Indicación del formato esperado, en el idioma del usuario.
        hint: String,
    },
    /// La fecha y hora no está escrita en el formato del idioma (`hint`, p. ej. `dd/mm/aaaa hh:mm`)
    /// ni en el formato ISO 8601, o no es una fecha y hora válida.
    #[error("invalid date and time, expected format {hint}")]
    InvalidDateTime {
        /// Indicación del formato esperado, en el idioma del usuario.
        hint: String,
    },
    /// La hora no existe en la zona horaria en la que se interpreta (la efectiva del documento)
    /// porque coincide con el adelanto del reloj al horario de verano (p. ej. las 02:30 del último
    /// domingo de marzo en Madrid).
    #[error("nonexistent local time due to a daylight saving time change")]
    NonexistentTime,
}

impl DateInputError {
    /// Devuelve un mensaje traducido para el usuario, con el formato esperado si procede.
    pub fn message(&self) -> Lc {
        match self {
            Self::InvalidDate { hint } => {
                Lc::l("date_input_invalid").with_arg("format", hint.clone())
            }
            Self::InvalidTime { hint } => {
                Lc::l("time_input_invalid").with_arg("format", hint.clone())
            }
            Self::InvalidDateTime { hint } => {
                Lc::l("datetime_input_invalid").with_arg("format", hint.clone())
            }
            Self::NonexistentTime => Lc::l("datetime_input_nonexistent"),
        }
    }
}
