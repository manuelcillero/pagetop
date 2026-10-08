use crate::locale::{LangId, Lc};

use super::{DateInputError, DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Tz, Utc};

use chrono::LocalResult;

// Patrones de respaldo (ISO 8601), sólo si faltase la clave Fluent del idioma (no debería ocurrir
// porque el idioma de respaldo `en-US` siempre las define).
const FALLBACK_DATE_FORMAT: &str = "%Y-%m-%d";
const FALLBACK_TIME_FORMAT: &str = "%H:%M";

// Formatos ISO 8601 que se aceptan siempre, además del propio del idioma, para no rechazar un valor
// pegado de otra fuente. No hay ambigüedad posible: empiezan por el año con cuatro cifras.
const ISO_DATE: &str = "%Y-%m-%d";
const ISO_TIME: &str = "%H:%M";
const ISO_DATETIME: [&str; 2] = ["%Y-%m-%dT%H:%M", "%Y-%m-%d %H:%M"];

// Años admitidos. `chrono` acepta `%Y` con cualquier número de cifras, así que "06/10/26" se leería
// como el año 26; se rechaza para que no pase inadvertido un año escrito con dos cifras.
const YEARS: std::ops::RangeInclusive<i32> = 1000..=9999;

// Patrón `strftime` de fecha del idioma, el mismo para mostrar y para leer (p. ej. `%d/%m/%Y`).
pub(crate) fn date_format(language: &impl LangId) -> String {
    Lc::l("date_input_format")
        .lookup(language)
        .unwrap_or_else(|| FALLBACK_DATE_FORMAT.to_string())
}

// Patrón `strftime` de hora del idioma (p. ej. `%H:%M`).
pub(crate) fn time_format(language: &impl LangId) -> String {
    Lc::l("time_input_format")
        .lookup(language)
        .unwrap_or_else(|| FALLBACK_TIME_FORMAT.to_string())
}

// Patrón de fecha y hora: el de fecha y el de hora separados por un espacio.
pub(crate) fn datetime_format(language: &impl LangId) -> String {
    let (date, time) = (date_format(language), time_format(language));
    crate::util::join!(date, " ", time)
}

// Indicación del formato para el usuario (p. ej. `dd/mm/aaaa`).
pub(crate) fn date_hint(language: &impl LangId) -> String {
    Lc::l("date_input_hint")
        .lookup(language)
        .unwrap_or_else(|| "yyyy-mm-dd".to_string())
}

// Indicación del formato de hora para el usuario (p. ej. `hh:mm`).
pub(crate) fn time_hint(language: &impl LangId) -> String {
    Lc::l("time_input_hint")
        .lookup(language)
        .unwrap_or_else(|| "hh:mm".to_string())
}

// Indicación del formato de fecha y hora para el usuario (p. ej. `dd/mm/aaaa hh:mm`).
pub(crate) fn datetime_hint(language: &impl LangId) -> String {
    let (date, time) = (date_hint(language), time_hint(language));
    crate::util::join!(date, " ", time)
}

// Lee una fecha con el formato del idioma o en ISO 8601. En blanco devuelve `None`.
pub(crate) fn parse_date(
    text: &str,
    language: &impl LangId,
) -> Result<Option<NaiveDate>, DateInputError> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    [date_format(language).as_str(), ISO_DATE]
        .into_iter()
        .find_map(|format| NaiveDate::parse_from_str(text, format).ok())
        .filter(|date| YEARS.contains(&chrono::Datelike::year(date)))
        .map(Some)
        .ok_or_else(|| DateInputError::InvalidDate {
            hint: date_hint(language),
        })
}

// Lee una hora con el formato del idioma o en ISO 8601. En blanco devuelve `None`.
pub(crate) fn parse_time(
    text: &str,
    language: &impl LangId,
) -> Result<Option<NaiveTime>, DateInputError> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    [time_format(language).as_str(), ISO_TIME]
        .into_iter()
        .find_map(|format| NaiveTime::parse_from_str(text, format).ok())
        .map(Some)
        .ok_or_else(|| DateInputError::InvalidTime {
            hint: time_hint(language),
        })
}

// Lee una fecha y hora local de `tz`, con el formato del idioma o en ISO 8601, y la convierte a
// UTC. En blanco devuelve `None`.
//
// En el cambio al horario de verano hay horas que no existen en `tz` (p. ej. las 02:30 del último
// domingo de marzo en Madrid): se rechazan, porque cualquier corrección inventaría una hora que el
// usuario no ha escrito. En el cambio al horario de invierno hay horas que existen dos veces: se
// toma la primera porque el usuario no tiene forma de indicar cuál de las dos quería.
pub(crate) fn parse_datetime(
    text: &str,
    language: &impl LangId,
    tz: Tz,
) -> Result<Option<DateTime<Utc>>, DateInputError> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    let format = datetime_format(language);
    let local = std::iter::once(format.as_str())
        .chain(ISO_DATETIME)
        .find_map(|format| NaiveDateTime::parse_from_str(text, format).ok())
        .filter(|local| YEARS.contains(&chrono::Datelike::year(local)))
        .ok_or_else(|| DateInputError::InvalidDateTime {
            hint: datetime_hint(language),
        })?;
    match tz.from_local_datetime(&local) {
        LocalResult::Single(dt) | LocalResult::Ambiguous(dt, _) => Ok(Some(dt.with_timezone(&Utc))),
        LocalResult::None => Err(DateInputError::NonexistentTime),
    }
}

// Muestra una fecha con el formato del idioma, tal como se lee después.
pub(crate) fn format_date(date: NaiveDate, language: &impl LangId) -> String {
    date.format(&date_format(language)).to_string()
}

// Muestra una hora con el formato del idioma, tal como se lee después.
pub(crate) fn format_time(time: NaiveTime, language: &impl LangId) -> String {
    time.format(&time_format(language)).to_string()
}

// Muestra un instante en `tz` con el formato del idioma, tal como se lee después.
pub(crate) fn format_datetime(dt: DateTime<Utc>, language: &impl LangId, tz: Tz) -> String {
    dt.with_timezone(&tz)
        .format(&datetime_format(language))
        .to_string()
}
