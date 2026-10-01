use crate::locale::{LangId, Lc};

use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};

// Nombre del mes (1-12) traducido contra `LOCALES_PAGETOP` para `language` (claves
// `month_01`..`month_12`). `chrono` sólo traduce `%B`/`%A` con la *feature* `unstable-locales`,
// redundante con Fluent y descartada para todo el proyecto; el nombre se resuelve aquí y se pasa
// como argumento `{ $month }` a `date_format_long` y, desde `precision.rs`, a `since_*`/`until_*`.
pub(crate) fn month_name(month: u32, language: &impl LangId) -> String {
    Lc::l(format!("month_{month:02}"))
        .lookup(language)
        .unwrap_or_else(|| month.to_string())
}

// **< DateFormat >*********************************************************************************

// Patrones `strftime` de respaldo, sólo si faltase la clave Fluent correspondiente (no debería
// ocurrir porque el idioma de respaldo `en-US` siempre las define).
const FALLBACK_DATE_SHORT: &str = "%d/%m/%y";
const FALLBACK_DATE_MEDIUM: &str = "%d/%m/%Y";
const FALLBACK_DATE_LONG: &str = "%d/%m/%Y";

// Fecha en formato ISO 8601; igual en todos los idiomas (no es una clave Fluent).
const ISO_DATE: &str = "%Y-%m-%d";

/// Tipo de formato a aplicar al mostrar una fecha **sin hora** ([`NaiveDate`](chrono::NaiveDate)).
///
/// `Short`, `Medium` y `Long` toman su texto de las claves Fluent
/// `date_format_short`/`_medium`/`_long` del idioma efectivo. `Iso` es un patrón ISO 8601 fijo,
/// igual en todos los idiomas (no pasa por Fluent porque nunca debería divergir entre idiomas).
/// `Custom` acepta un patrón `strftime` explícito para el resto de casos.
///
/// Para una fecha y hora completas, combina esta fecha con [`TimeFormat`] mediante
/// [`Contextual::format_datetime()`](crate::core::component::Contextual::format_datetime).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn show(cx: &Context, date: NaiveDate) {
/// let short = cx.format_date(date, DateFormat::Short);
/// let long = cx.format_date(date, DateFormat::Long); // p. ej. "15 de junio de 2026"
/// let iso = cx.format_date(date, DateFormat::Iso);
/// let custom = cx.format_date(date, DateFormat::Custom("%Y-%m-%d"));
/// # }
/// ```
#[derive(Clone, Copy, Debug)]
pub enum DateFormat<'a> {
    /// Fecha corta.
    Short,
    /// Fecha con año completo.
    Medium,
    /// Fecha larga (formato extenso propio del idioma), con el nombre del mes.
    Long,
    /// Fecha en formato ISO 8601; igual en todos los idiomas.
    Iso,
    /// Patrón `strftime` explícito.
    Custom(&'a str),
}

impl DateFormat<'_> {
    // Formatea `date` para `language`.
    pub(crate) fn apply(&self, date: NaiveDate, language: &impl LangId) -> String {
        match self {
            DateFormat::Short => Lc::l("date_format_short")
                .with_arg("day", format!("{:02}", date.day()))
                .with_arg("month", format!("{:02}", date.month()))
                .with_arg("year_short", format!("{:02}", date.year().rem_euclid(100)))
                .lookup(language)
                .unwrap_or_else(|| date.format(FALLBACK_DATE_SHORT).to_string()),
            DateFormat::Medium => Lc::l("date_format_medium")
                .with_arg("day", format!("{:02}", date.day()))
                .with_arg("month", format!("{:02}", date.month()))
                .with_arg("year", date.year().to_string())
                .lookup(language)
                .unwrap_or_else(|| date.format(FALLBACK_DATE_MEDIUM).to_string()),
            DateFormat::Long => Lc::l("date_format_long")
                .with_arg("day", date.day().to_string())
                .with_arg("month", month_name(date.month(), language))
                .with_arg("year", date.year().to_string())
                .lookup(language)
                .unwrap_or_else(|| date.format(FALLBACK_DATE_LONG).to_string()),
            DateFormat::Iso => date.format(ISO_DATE).to_string(),
            DateFormat::Custom(pattern) => date.format(pattern).to_string(),
        }
    }
}

// **< TimeFormat >*********************************************************************************

// Patrones `strftime` de respaldo, sólo si faltase la clave Fluent correspondiente.
const FALLBACK_TIME_SHORT: &str = "%H:%M";
const FALLBACK_TIME_LONG: &str = "%H:%M:%S";

/// Tipo de formato a aplicar al mostrar una hora del día ([`NaiveTime`](chrono::NaiveTime)).
///
/// Mismo mecanismo que [`DateFormat`], en este caso usa `Short` y `Long` para tomar su texto de las
/// claves Fluent `time_format_short`/`_long` del idioma efectivo, aplicando plantillas con los
/// argumentos `{ $hour }`, `{ $minute }` (más `{ $second }` en `Long`). `Custom` acepta un patrón
/// `strftime` explícito, útil para aplicar, por ejemplo, un formato de 12 horas con AM/PM
/// independientemente del idioma efectivo (`"%I:%M %p"`).
///
/// Para una fecha y hora completas, combina este `TimeFormat` con un [`DateFormat`] mediante
/// [`Contextual::format_datetime()`], que permite elegir un formato distinto para cada parte (p.
/// ej. fecha larga con hora corta) en vez de un único formato combinado fijo.
///
/// [`Contextual::format_datetime()`]: crate::core::component::Contextual::format_datetime
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn show(cx: &Context, dt: DateTime<Utc>) {
/// let short = cx.format_time(dt, TimeFormat::Short);
/// let long = cx.format_time(dt, TimeFormat::Long);
/// let custom = cx.format_time(dt, TimeFormat::Custom("%H:%M:%S"));
///
/// // Formato de 12 horas con AM/PM (p. ej. "02:30 PM").
/// let twelve_hour = cx.format_time(dt, TimeFormat::Custom("%I:%M %p"));
/// # }
/// ```
#[derive(Clone, Copy, Debug)]
pub enum TimeFormat<'a> {
    /// Hora y minutos.
    Short,
    /// Hora, minutos y segundos.
    Long,
    /// Patrón `strftime` explícito.
    Custom(&'a str),
}

impl TimeFormat<'_> {
    // Formatea `time` para `language`.
    pub(crate) fn apply(&self, time: NaiveTime, language: &impl LangId) -> String {
        match self {
            TimeFormat::Short => Lc::l("time_format_short")
                .with_arg("hour", format!("{:02}", time.hour()))
                .with_arg("minute", format!("{:02}", time.minute()))
                .lookup(language)
                .unwrap_or_else(|| time.format(FALLBACK_TIME_SHORT).to_string()),
            TimeFormat::Long => Lc::l("time_format_long")
                .with_arg("hour", format!("{:02}", time.hour()))
                .with_arg("minute", format!("{:02}", time.minute()))
                .with_arg("second", format!("{:02}", time.second()))
                .lookup(language)
                .unwrap_or_else(|| time.format(FALLBACK_TIME_LONG).to_string()),
            TimeFormat::Custom(pattern) => time.format(pattern).to_string(),
        }
    }
}
