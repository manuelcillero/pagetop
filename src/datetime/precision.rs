use super::format::month_name;
use crate::locale::{LangId, Lc};

use chrono::{Datelike, NaiveDate};

/// Precisión a aplicar al mostrar una fecha de inicio o fin.
///
/// Por ejemplo, "desde junio" o "hasta junio de 2026", usada por [`Contextual::format_since()`] y
/// [`Contextual::format_until()`].
///
/// A diferencia de [`DateFormat`], que siempre muestra día, mes y año (sólo cambia el estilo),
/// aquí lo que cambia es la propia precisión revelada, dejando `Short` para sólo el mes, `Medium`
/// para mes y año, y `Long` para la fecha completa. El texto de cada nivel es una plantilla
/// Fluent completa por idioma (traducciones `since_short`/`since_medium`/`since_long` y
/// `until_short`/`until_medium`/`until_long` en `src/locale/{lang}/datetime.ftl`). En español, por
/// ejemplo, el artículo "el" sólo aparece delante de un día concreto ("desde el 3 de junio de
/// 2026", pero "desde junio de 2026" no lleva "el"), así que forma parte de la plantilla de `Long`,
/// no de un prefijo genérico compartido.
///
/// No tiene variante `Custom` porque no hay un patrón `strftime` equivalente para "sólo el mes" o
/// "mes y año" con el nombre del mes traducido (mismo motivo que en [`RelativeFormat`]).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn show(cx: &Context, start: NaiveDate, end: NaiveDate) {
/// let since = cx.format_since(start, DatePrecision::Medium); // p. ej. "desde junio de 2026"
/// let until = cx.format_until(end, DatePrecision::Long); // p. ej. "hasta el 3 de junio de 2026"
/// # }
/// ```
///
/// [`Contextual::format_since()`]: crate::core::component::Contextual::format_since
/// [`Contextual::format_until()`]: crate::core::component::Contextual::format_until
/// [`DateFormat`]: super::DateFormat
/// [`RelativeFormat`]: super::RelativeFormat
#[derive(Clone, Copy, Debug)]
pub enum DatePrecision {
    /// Sólo el mes.
    Short,
    /// Mes y año.
    Medium,
    /// Día, mes y año.
    Long,
}

impl DatePrecision {
    // Formatea `date` para `language` contra el trío de claves `since_short`/`_medium`/`_long`.
    pub(crate) fn apply_since(&self, date: NaiveDate, language: &impl LangId) -> String {
        self.apply(
            date,
            ["since_short", "since_medium", "since_long"],
            language,
        )
    }

    // Formatea `date` para `language` contra el trío de claves `until_short`/`_medium`/`_long`.
    pub(crate) fn apply_until(&self, date: NaiveDate, language: &impl LangId) -> String {
        self.apply(
            date,
            ["until_short", "until_medium", "until_long"],
            language,
        )
    }

    // `apply_since()`/`apply_until()` comparten exactamente los mismos argumentos por nivel de
    // precisión; sólo cambia el trío de claves Fluent que se resuelve.
    fn apply(&self, date: NaiveDate, keys: [&'static str; 3], language: &impl LangId) -> String {
        let [short, medium, long] = keys;
        let month = || month_name(date.month(), language);
        match self {
            DatePrecision::Short => Lc::l(short)
                .with_arg("month", month())
                .lookup(language)
                .unwrap_or_else(month),
            DatePrecision::Medium => Lc::l(medium)
                .with_arg("month", month())
                .with_arg("year", date.year().to_string())
                .lookup(language)
                .unwrap_or_else(|| format!("{} {}", month(), date.year())),
            DatePrecision::Long => Lc::l(long)
                .with_arg("day", date.day().to_string())
                .with_arg("month", month())
                .with_arg("year", date.year().to_string())
                .lookup(language)
                .unwrap_or_else(|| format!("{} {} {}", date.day(), month(), date.year())),
        }
    }
}
