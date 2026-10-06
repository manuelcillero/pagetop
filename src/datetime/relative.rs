use crate::locale::{LangId, Lc};

use chrono::{Months, NaiveDate};

/// Tipo de formato a aplicar al mostrar una fecha **en relación al momento actual**.
///
/// Por ejemplo, "hace 3 años" o "dentro de 5 días", en vez de la fecha absoluta.
///
/// El desglose es un cálculo exacto de calendario en años, meses y días (no una duración
/// redondeada tipo "hace unos 3 años"): `Short` muestra el componente más significativo, `Medium`
/// hasta dos y `Long` hasta tres. Los componentes en cero se omiten antes de aplicar el límite, no
/// después: con una diferencia de "2 años, 0 meses y 3 días", tanto `Medium` como `Long` muestran
/// "2 años y 3 días" (sin un componente de meses que mostrar, `Long` no tiene un tercero disponible
/// y coincide con `Medium`).
///
/// La unidad mínima es el día: la comparación se hace entre fechas civiles (la de `dt`, convertida
/// a la zona horaria efectiva, y la de "ahora"), no entre instantes. Un valor de hace unos minutos o
/// dentro de unos minutos, si cae en la misma fecha civil que "ahora", se muestra como "hoy"
/// (clave Fluent `relative_today`), sin necesidad de ningún umbral de minutos u horas.
///
/// El texto completo ("hace 3 años y 2 meses" / "3 years and 2 months ago") se compone enteramente
/// a partir de claves Fluent del idioma efectivo (`src/locale/{lang}/datetime.ftl`): el orden y la
/// posición de "hace"/"dentro de" (prefijo en español, "ago" como sufijo en inglés), la unión de
/// varios componentes y el plural de cada unidad son propiedades del idioma, no de este tipo.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # fn show(cx: &Context, dt: DateTime<Utc>) {
/// let short = cx.format_relative(dt, RelativeFormat::Short); // p. ej. "hace 3 años"
/// let long = cx.format_relative(dt, RelativeFormat::Long); // p. ej. "hace 3 años, 2 meses y 10 días"
/// # }
/// ```
#[derive(Clone, Copy, Debug)]
pub enum RelativeFormat {
    /// El componente más significativo (años, meses o días).
    Short,
    /// Hasta dos componentes.
    Medium,
    /// Hasta tres componentes (años, meses y días).
    Long,
}

impl RelativeFormat {
    // Compara `target` con `today` (ambas fechas civiles ya en la zona horaria efectiva) y compone
    // el texto relativo para `language`.
    pub(crate) fn apply(
        &self,
        target: NaiveDate,
        today: NaiveDate,
        language: &impl LangId,
    ) -> String {
        if target == today {
            return Lc::l("relative_today")
                .lookup(language)
                .unwrap_or_else(|| "today".to_owned());
        }

        let is_future = target > today;
        let (early, late) = if is_future {
            (today, target)
        } else {
            (target, today)
        };
        let (years, months, days) = breakdown(early, late);

        let mut parts = Vec::with_capacity(3);
        if years > 0 {
            parts.push(unit("relative_years", years, language));
        }
        if months > 0 {
            parts.push(unit("relative_months", months, language));
        }
        if days > 0 {
            parts.push(unit("relative_days", days, language));
        }
        let depth = match self {
            RelativeFormat::Short => 1,
            RelativeFormat::Medium => 2,
            RelativeFormat::Long => 3,
        };
        parts.truncate(depth);

        let value = join(&parts, language);
        let key = if is_future {
            "relative_future"
        } else {
            "relative_past"
        };
        Lc::l(key)
            .with_arg("value", value.clone())
            .lookup(language)
            .unwrap_or(value)
    }
}

// **< HELPERS >************************************************************************************

// Descompone el intervalo `[early, late]` (`early <= late`, garantizado por el llamador) en años,
// meses y días completos de calendario, no en una duración aproximada.
//
// Se apoya en `NaiveDate::checked_add_months()`. Cuando el día de partida no existe en el mes de
// destino (p. ej. 31 de enero + 1 mes, y febrero no llega a 31), ajusta al último día válido de ese
// mes (28 de febrero, o 29 si es bisiesto).
fn breakdown(early: NaiveDate, late: NaiveDate) -> (u32, u32, u32) {
    let mut months = 0u32;
    let mut cursor = early;
    while let Some(next) = early.checked_add_months(Months::new(months + 1)) {
        if next > late {
            break;
        }
        months += 1;
        cursor = next;
    }
    let days = (late - cursor).num_days() as u32;
    (months / 12, months % 12, days)
}

// Texto de un componente ("3 años", "1 mes"...), con selección de plural real vía `with_number()`.
fn unit(key: &'static str, n: u32, language: &impl LangId) -> String {
    Lc::l(key)
        .with_number("n", n)
        .lookup(language)
        .unwrap_or_else(|| n.to_string())
}

// Une de 1 a 3 componentes ya traducidos en una frase natural del idioma ("3 años, 2 meses y 10
// días"). El punto de unión (con o sin coma, con o sin conjunción final) es una propiedad del
// idioma, igual que `datetime_join`.
fn join(parts: &[String], language: &impl LangId) -> String {
    match parts {
        [] => String::new(),
        [a] => a.clone(),
        [a, b] => Lc::l("relative_join_two")
            .with_arg("a", a.clone())
            .with_arg("b", b.clone())
            .lookup(language)
            .unwrap_or_else(|| format!("{a} {b}")),
        [a, b, c, ..] => Lc::l("relative_join_three")
            .with_arg("a", a.clone())
            .with_arg("b", b.clone())
            .with_arg("c", c.clone())
            .lookup(language)
            .unwrap_or_else(|| format!("{a} {b} {c}")),
    }
}

// `breakdown()` y `apply()` dependen de fechas exactas de calendario (bisiestos, fin de mes,
// componentes en cero intercalados...), no de la hora actual. Se prueban aquí, con fechas fijas, en
// vez de en `tests/datetime.rs` (que sólo ve la API pública `Contextual::format_relative()`,
// siempre relativa a `Utc::now()` real y no serviría para fijar estos casos concretos).
#[cfg(test)]
mod tests {
    use super::*;
    use crate::locale::Locale;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn breakdown_splits_years_months_days() {
        assert_eq!(breakdown(date(2020, 3, 5), date(2023, 5, 15)), (3, 2, 10));
    }

    #[test]
    fn breakdown_skips_to_days_when_months_are_zero() {
        assert_eq!(breakdown(date(2022, 6, 12), date(2024, 6, 15)), (2, 0, 3));
    }

    #[test]
    fn breakdown_clamps_month_end_across_a_leap_year() {
        // Jan 31, 2024 (leap year) + 1 month = Feb 29 (checked_add_months clamps it); from there to
        // Mar 1 there is 1 more day: 1 month and 1 day, not "1 month and -1 day" nor "2 months".
        assert_eq!(breakdown(date(2024, 1, 31), date(2024, 3, 1)), (0, 1, 1));
    }

    #[test]
    fn apply_same_civil_date_is_today() {
        let today = date(2026, 6, 15);
        let en = Locale::resolve("en-US");
        assert_eq!(RelativeFormat::Short.apply(today, today, &en), "today");
    }

    #[test]
    fn apply_truncates_after_filtering_zero_components() {
        let today = date(2026, 6, 15);
        let target = date(2024, 6, 12); // 2 years, 0 months, 3 days.
        let en = Locale::resolve("en-US");

        assert_eq!(
            RelativeFormat::Short.apply(target, today, &en),
            "2 years ago"
        );
        assert_eq!(
            RelativeFormat::Medium.apply(target, today, &en),
            "2 years and 3 days ago"
        );
        // With no third component available (months = 0), `Long` matches `Medium`.
        assert_eq!(
            RelativeFormat::Long.apply(target, today, &en),
            "2 years and 3 days ago"
        );
    }

    #[test]
    fn apply_shows_three_components_in_spanish() {
        let today = date(2026, 6, 15);
        let target = date(2023, 4, 5); // 3 years, 2 months, 10 days.
        let es = Locale::resolve("es-ES");

        assert_eq!(
            RelativeFormat::Long.apply(target, today, &es),
            "hace 3 años, 2 meses y 10 días"
        );
    }

    #[test]
    fn apply_handles_future_dates_and_singular_units() {
        let today = date(2026, 6, 15);
        let target = date(2026, 6, 16); // in 1 day.
        let es = Locale::resolve("es-ES");

        assert_eq!(
            RelativeFormat::Short.apply(target, today, &es),
            "dentro de 1 día"
        );
    }
}
