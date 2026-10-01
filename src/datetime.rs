//! Soporte a fechas y horas según estándar [ISO 8601].
//!
//! Basado en [chrono] y zonas horarias de [chrono-tz] con formatos de visualización según idioma.
//!
//! Las fechas y horas se graban siempre en UTC y se muestran en la zona horaria efectiva del
//! usuario actual ([`CurrentUser::timezone()`]), ya sea la suya propia si tiene una configurada y
//! válida; y si no, la configurada para la aplicación ([`Timezone`]); o, en su defecto, UTC. La
//! conversión sólo ocurre al mostrar ([`Contextual::format_datetime()`]), nunca al guardar.
//!
//! [`DateFormat`] (fecha) y [`TimeFormat`] (hora) son tipos independientes: no existe un formato
//! combinado de fecha y hora. Para mostrar ambas, [`Contextual::format_datetime()`] junta el
//! resultado de cada uno (posiblemente con formatos distintos, p. ej. fecha larga con hora corta)
//! mediante la clave Fluent `datetime_join` del idioma efectivo. El orden día/mes/año, el nombre del
//! mes y el separador de fecha y hora son una propiedad del idioma, no de la configuración de la
//! aplicación (a diferencia de la zona horaria). Se resuelven como claves Fluent normales
//! (`src/locale/{lang}/datetime.ftl`), con el mismo mecanismo que cualquier otro texto traducido de
//! PageTop.
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
//! [`CurrentUser::timezone()`]: crate::auth::CurrentUser::timezone
//! [`Contextual::format_datetime()`]: crate::core::component::Contextual::format_datetime
//! [`Contextual::format_relative()`]: crate::core::component::Contextual::format_relative
//! [`Contextual::format_since()`]: crate::core::component::Contextual::format_since
//! [`Contextual::format_until()`]: crate::core::component::Contextual::format_until

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

mod format;
pub use format::{DateFormat, TimeFormat};

mod relative;
pub use relative::RelativeFormat;

mod precision;
pub use precision::DatePrecision;
