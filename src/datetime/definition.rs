use crate::global::{self, TimezoneOrder};
use crate::{trace, util};

use super::{DateTime, TZ_VARIANTS, TimeZone, Tz, TzRegion};

use chrono::Offset;

use std::collections::BTreeMap;
use std::f64::consts::TAU;
use std::sync::LazyLock;

// Valor de `app.timezone` sin espacios al principio ni al final, si no está vacío.
fn configured_raw() -> Option<&'static str> {
    global::SETTINGS
        .app
        .timezone
        .as_deref()
        .and_then(util::non_blank)
}

// Zona horaria configurada para la aplicación, si es válida. El nombre se busca sin distinguir
// mayúsculas y minúsculas, como las regiones de `app.timezone_regions`.
static CONFIG_TZ: LazyLock<Option<Tz>> = LazyLock::new(|| {
    configured_raw().and_then(|raw| {
        TZ_VARIANTS
            .iter()
            .copied()
            .find(|tz| tz.name().eq_ignore_ascii_case(raw))
    })
});

// Zona horaria de respaldo, garantizada incluso sin configuración válida.
const FALLBACK_TZ: Tz = Tz::UTC;

// Regiones válidas de `app.timezone_regions`; los nombres desconocidos se ignoran (ver `init()`).
static CONFIG_REGIONS: LazyLock<Vec<TzRegion>> = LazyLock::new(|| {
    global::SETTINGS
        .app
        .timezone_regions
        .split(',')
        .filter_map(TzRegion::from_name)
        .collect()
});

// Zonas horarias que se ofrecen para elegir según la configuración (ver `regions_by()`).
static TZ_BY_REGION: LazyLock<Vec<(TzRegion, &'static [&'static str])>> =
    LazyLock::new(|| Timezone::regions_with(global::SETTINGS.app.timezone_order, &CONFIG_REGIONS));

// Alias de la base IANA que no se ofrecen porque repiten otra zona que sí se ofrece: nombres
// antiguos (`Australia/NSW`), grafías alternativas (`Asia/Calcutta` es `Asia/Kolkata`) y lugares
// que duplican a otro sin representar a ningún país (`America/Montreal` es `America/Toronto`).
//
// Son los enlaces de cuatro secciones del fichero `backward` de tzdata 2025b (el que incluye
// `chrono-tz`): "Pre-1993 naming conventions", "Two-part names that were renamed...", "Non-zone.tab
// locations..." y "Alternate names for the same location"; salvo `Asia/Istanbul` y
// `Europe/Nicosia`, que la IANA mantiene para encontrar Turquía y Chipre en los dos continentes. Se
// conservan los de la sección "Pre-2013 practice" (`Europe/Oslo`, `Africa/Accra`...); son zonas
// fusionadas con otra porque coinciden desde 1970, pero cada una es la referencia de su país.
//
// Revisar al actualizar `chrono-tz`. Si quedara desfasada, reaparecería algún duplicado o se
// ocultaría una zona que hubiera dejado de ser alias; el test avisa si alguno deja de existir.
const ALIAS_ZONES: &[&str] = &[
    "Africa/Asmera",
    "Africa/Timbuktu",
    "America/Argentina/ComodRivadavia",
    "America/Atka",
    "America/Buenos_Aires",
    "America/Catamarca",
    "America/Coral_Harbour",
    "America/Cordoba",
    "America/Ensenada",
    "America/Fort_Wayne",
    "America/Godthab",
    "America/Indianapolis",
    "America/Jujuy",
    "America/Knox_IN",
    "America/Louisville",
    "America/Mendoza",
    "America/Montreal",
    "America/Nipigon",
    "America/Pangnirtung",
    "America/Porto_Acre",
    "America/Rainy_River",
    "America/Rosario",
    "America/Santa_Isabel",
    "America/Shiprock",
    "America/Thunder_Bay",
    "America/Virgin",
    "America/Yellowknife",
    "Antarctica/South_Pole",
    "Asia/Ashkhabad",
    "Asia/Calcutta",
    "Asia/Choibalsan",
    "Asia/Chongqing",
    "Asia/Chungking",
    "Asia/Dacca",
    "Asia/Harbin",
    "Asia/Kashgar",
    "Asia/Katmandu",
    "Asia/Macao",
    "Asia/Rangoon",
    "Asia/Saigon",
    "Asia/Tel_Aviv",
    "Asia/Thimbu",
    "Asia/Ujung_Pandang",
    "Asia/Ulan_Bator",
    "Atlantic/Faeroe",
    "Atlantic/Jan_Mayen",
    "Australia/ACT",
    "Australia/Canberra",
    "Australia/Currie",
    "Australia/LHI",
    "Australia/NSW",
    "Australia/North",
    "Australia/Queensland",
    "Australia/South",
    "Australia/Tasmania",
    "Australia/Victoria",
    "Australia/West",
    "Australia/Yancowinna",
    "Europe/Belfast",
    "Europe/Kiev",
    "Europe/Tiraspol",
    "Europe/Uzhgorod",
    "Europe/Zaporozhye",
    "Pacific/Enderbury",
    "Pacific/Johnston",
    "Pacific/Ponape",
    "Pacific/Samoa",
    "Pacific/Truk",
    "Pacific/Yap",
];

// Región de zonas horarias con sus zonas ordenadas por nombre y el ángulo de su desfase medio en un
// reloj de 24 horas (ver `angle()`).
struct Region {
    region: TzRegion,
    zones: Vec<&'static str>,
    center: f64,
}

// Todas las regiones que se pueden ofrecer, ordenadas por nombre. Sólo se conservan las zonas de
// alguna región de `TzRegion`, lo que descarta los nombres sin región (alias heredados como `GB`,
// `Japan` o `EST5EDT`) y las regiones formadas sólo por alias heredados (`US/Eastern` es
// `America/New_York`). De `Etc` sólo se conserva `Etc/UTC`: el resto son alias de UTC
// (`Etc/Zulu`...) o zonas de desfase fijo (`Etc/GMT+1`...) que no representan ningún lugar.
// También se descartan los alias de `ALIAS_ZONES`: `chrono-tz` no distingue zonas canónicas de
// enlaces.
//
// Sin coordenadas en `chrono-tz`, la posición de cada región es el desfase medio de sus zonas,
// tratado como un ángulo (media circular) porque `Pacific` abarca de -11 a +14 horas y su centro
// real está junto a la línea de cambio de fecha, no a medio camino entre ambos extremos.
static ALL_REGIONS: LazyLock<Vec<Region>> = LazyLock::new(|| {
    // Por región: nombres de sus zonas y suma de senos y cosenos de sus desfases.
    let mut regions: BTreeMap<TzRegion, (Vec<&'static str>, f64, f64)> = BTreeMap::new();
    for tz in TZ_VARIANTS.iter() {
        let name = tz.name();
        let Some(region) = name
            .split_once('/')
            .and_then(|(region, _)| TzRegion::from_name(region))
        else {
            continue;
        };
        if (region == TzRegion::Etc && name != "Etc/UTC")
            || ALIAS_ZONES.binary_search(&name).is_ok()
        {
            continue;
        }
        let (zones, sin, cos) = regions.entry(region).or_default();
        let a = angle(tz);
        zones.push(name);
        *sin += a.sin();
        *cos += a.cos();
    }
    regions
        .into_iter()
        .map(|(region, (mut zones, sin, cos))| {
            zones.sort_unstable();
            Region {
                region,
                zones,
                center: sin.atan2(cos),
            }
        })
        .collect()
});

// Desfase de una zona horaria respecto a UTC como ángulo de un reloj de 24 horas. Se toma en un
// instante fijo (15/01/2026) para que el orden no dependa de la fecha; el horario de verano movería
// los desfases normalmente una hora.
fn angle(tz: &Tz) -> f64 {
    let at = DateTime::from_timestamp(1_768_435_200, 0)
        .unwrap_or_default()
        .naive_utc();
    let seconds = tz.offset_from_utc_datetime(&at).fix().local_minus_utc();
    f64::from(seconds) / 86_400.0 * TAU
}

// Si la región se ofrece con la lista dada: con una lista vacía se ofrecen todas; si no, sólo las
// indicadas y `Etc`, que se mantiene siempre porque `Etc/UTC` equivale a `UTC`, la zona horaria de
// respaldo.
fn is_offered(region: TzRegion, list: &[TzRegion]) -> bool {
    list.is_empty() || region == TzRegion::Etc || list.contains(&region)
}

// Regiones que se ofrecen con la lista dada (ver `is_offered()`), ordenadas según `order`. `Etc` va
// siempre al final. `site` es la referencia de `Nearest`, que mide la cercanía con media y
// distancia circulares.
fn regions_by(order: TimezoneOrder, list: &[TzRegion], site: Tz) -> Vec<&'static Region> {
    let site_region = site
        .name()
        .split_once('/')
        .and_then(|(region, _)| TzRegion::from_name(region));
    let site_angle = angle(&site);
    let mut sorted: Vec<_> = ALL_REGIONS
        .iter()
        .filter(|r| is_offered(r.region, list))
        .map(|r| {
            // Grupo (0 primero, 2 al final) y clave de orden dentro del grupo; empates por nombre.
            let (rank, key) = match order {
                _ if r.region == TzRegion::Etc => (2, 0.0),
                TimezoneOrder::Nearest if Some(r.region) == site_region => (0, 0.0),
                TimezoneOrder::Nearest => {
                    let gap = (r.center - site_angle).rem_euclid(TAU);
                    (1, gap.min(TAU - gap))
                }
                TimezoneOrder::Alphabetical => (1, 0.0),
                TimezoneOrder::Listed => {
                    let position = list.iter().position(|listed| *listed == r.region);
                    (1, position.map_or(0.0, |p| p as f64))
                }
            };
            (rank, key, r)
        })
        .collect();
    sorted.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.1.total_cmp(&b.1))
            .then(a.2.region.as_str().cmp(b.2.region.as_str()))
    });
    sorted.into_iter().map(|(_, _, r)| r).collect()
}

/// Zona horaria configurada para la aplicación y zonas horarias para elegir.
///
/// Resuelve [`global::SETTINGS.app.timezone`] contra la base IANA de zonas horarias. Si no se ha
/// configurado o el valor no es válido, se aplica la zona horaria de respaldo (`UTC`). Ofrece
/// además las zonas horarias que se pueden elegir ([`supported_by_region()`],
/// [`is_supported_in()`]).
///
/// [`global::SETTINGS.app.timezone`]: crate::global::App::timezone
/// [`supported_by_region()`]: Self::supported_by_region
/// [`is_supported_in()`]: Self::is_supported_in
pub struct Timezone;

impl Timezone {
    // Inicializa la zona horaria por defecto y las regiones para elegir.
    //
    // Debe llamarse durante la inicialización para indicar si la zona horaria por defecto procede
    // de la configuración, de una configuración no válida o de la zona horaria de respaldo, y para
    // avisar de las regiones de `app.timezone_regions` que no existan. De paso, calcula al arrancar
    // las zonas horarias que se ofrecen, en vez de esperar a la primera petición que las necesite.
    pub(crate) fn init() {
        trace::debug!(
            "Timezone database: IANA tzdata {}",
            chrono_tz::IANA_TZDB_VERSION
        );
        match configured_raw() {
            Some(raw) => {
                if let Some(tz) = *CONFIG_TZ {
                    trace::debug!("Default timezone \"{tz}\" (from config: \"{raw}\")");
                } else {
                    trace::warn!(
                        "Default timezone \"{FALLBACK_TZ}\" (fallback, invalid config: \"{raw}\")"
                    );
                }
            }
            None => trace::debug!("Default timezone \"{FALLBACK_TZ}\" (fallback, no config)"),
        }

        for name in global::SETTINGS.app.timezone_regions.split(',') {
            if !name.trim().is_empty() && TzRegion::from_name(name).is_none() {
                trace::warn!("Ignored unknown timezone region \"{}\"", name.trim());
            }
        }
        let regions: Vec<_> = Self::supported_by_region()
            .iter()
            .map(|(region, _)| region.as_str())
            .collect();
        let order = global::SETTINGS.app.timezone_order;
        trace::debug!(
            "Timezone regions offered ({order:?}): {}",
            regions.join(", ")
        );
    }

    /// Devuelve la zona horaria configurada explícitamente, si es válida.
    ///
    /// Si no se ha configurado una zona horaria por defecto o el valor no es válido, devuelve
    /// `None`.
    pub fn try_tz() -> Option<Tz> {
        *CONFIG_TZ
    }

    /// Devuelve siempre la zona horaria de respaldo (`UTC`).
    ///
    /// Es la zona horaria garantizada incluso cuando no haya configuración de la aplicación o
    /// cuando el valor configurado no sea válido.
    pub fn fallback_tz() -> Tz {
        FALLBACK_TZ
    }

    /// Devuelve la zona horaria configurada o, en su defecto, la de respaldo (`UTC`).
    pub fn default_tz() -> Tz {
        Self::try_tz().unwrap_or(FALLBACK_TZ)
    }

    /// Devuelve las zonas horarias IANA que se ofrecen para elegir, agrupadas por región.
    ///
    /// Cada grupo es la región ([`TzRegion`]) con los nombres completos de sus zonas (p. ej.
    /// `"Europe/Madrid"`) ordenados por nombre. Las regiones siguen el orden de
    /// [`global::SETTINGS.app.timezone_order`]: por defecto, primero la región de la zona horaria
    /// del sitio, si la tiene, y luego el resto, de la más cercana a la más lejana; con
    /// *"Alphabetical"*, o *"Listed"* sin regiones, se ordenan por su nombre IANA, y quien las
    /// muestre traducidas debe reordenarlas por el nombre traducido, como hace
    /// [`form::SelectTimezone`]. [`TzRegion::Etc`], sólo con `"Etc/UTC"`, va siempre al final.
    ///
    /// Se excluyen los nombres sin región (`"UTC"`, `"Japan"`...), las regiones formadas sólo por
    /// alias heredados (`"US"`, `"Canada"`...), los alias que sólo repiten otra zona con otro
    /// nombre (`"Asia/Calcutta"` es `"Asia/Kolkata"`), aunque se conservan las zonas de referencia
    /// de cada país fusionadas con otra (`"Europe/Oslo"`), y las zonas de desfase fijo
    /// (`"Etc/GMT+1"`...). Se limita a las regiones indicadas en
    /// [`global::SETTINGS.app.timezone_regions`], si las hay, más [`TzRegion::Etc`], que se ofrece
    /// siempre.
    ///
    /// Son las zonas horarias que ofrece por defecto [`form::SelectTimezone`], útiles también para
    /// validar el valor recibido, aceptando además, sin cambios, el que ya estuviera guardado
    /// aunque ya no se ofrezca.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let is_supported = |name: &str| {
    ///     Timezone::supported_by_region()
    ///         .iter()
    ///         .any(|(_, names)| names.contains(&name))
    /// };
    ///
    /// assert!(is_supported("Europe/Madrid"));
    /// assert!(!is_supported("US/Eastern"));
    /// ```
    ///
    /// [`global::SETTINGS.app.timezone_order`]: crate::global::App::timezone_order
    /// [`global::SETTINGS.app.timezone_regions`]: crate::global::App::timezone_regions
    /// [`form::SelectTimezone`]: crate::base::component::form::SelectTimezone
    pub fn supported_by_region() -> &'static [(TzRegion, &'static [&'static str])] {
        &TZ_BY_REGION
    }

    /// Devuelve si `name` es una de las zonas horarias que se ofrecen para elegir con las regiones
    /// indicadas: las de esas regiones más `Etc/UTC`, que se ofrece siempre, o las de todas si la
    /// lista está vacía. `name` debe ser el nombre IANA exacto, sin espacios y con las mayúsculas
    /// correctas.
    ///
    /// Sirve para validar el valor recibido de un [`form::SelectTimezone`] al que se le hayan
    /// cambiado las regiones con `with_regions()`; con las de la configuración basta
    /// [`supported_by_region()`]. En ambos casos debe aceptarse también, sin cambios, el valor que
    /// ya estuviera guardado aunque ya no se ofrezca.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let europe = [TzRegion::Europe];
    /// assert!(Timezone::is_supported_in("Europe/Madrid", &europe));
    /// assert!(Timezone::is_supported_in("Etc/UTC", &europe));
    /// assert!(!Timezone::is_supported_in("Asia/Tokyo", &europe));
    /// assert!(Timezone::is_supported_in("Asia/Tokyo", &[]));
    /// ```
    ///
    /// [`form::SelectTimezone`]: crate::base::component::form::SelectTimezone
    /// [`supported_by_region()`]: Self::supported_by_region
    pub fn is_supported_in(name: impl AsRef<str>, regions: &[TzRegion]) -> bool {
        let name = name.as_ref();
        ALL_REGIONS
            .iter()
            .any(|r| is_offered(r.region, regions) && r.zones.binary_search(&name).is_ok())
    }

    // Regiones de `app.timezone_regions`, valor por defecto de `SelectTimezone::with_regions()`.
    pub(crate) fn configured_regions() -> Vec<TzRegion> {
        CONFIG_REGIONS.clone()
    }

    // Regiones que se ofrecen con el orden y la lista dados; `supported_by_region()` es el
    // resultado con los de la configuración.
    pub(crate) fn regions_with(
        order: TimezoneOrder,
        regions: &[TzRegion],
    ) -> Vec<(TzRegion, &'static [&'static str])> {
        regions_by(order, regions, Self::default_tz())
            .into_iter()
            .map(|r| (r.region, r.zones.as_slice()))
            .collect()
    }

    // Si las regiones se ordenan por nombre: con `Alphabetical`, o con `Listed` sin lista. Se
    // ordenan por su nombre IANA, pero quien las muestre traducidas debe reordenarlas por el nombre
    // traducido.
    pub(crate) fn regions_by_name(order: TimezoneOrder, regions: &[TzRegion]) -> bool {
        match order {
            TimezoneOrder::Nearest => false,
            TimezoneOrder::Alphabetical => true,
            TimezoneOrder::Listed => regions.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use TimezoneOrder::{Alphabetical, Listed, Nearest};
    use TzRegion::*;

    fn tz(name: &str) -> Tz {
        name.parse().unwrap()
    }

    fn order(by: TimezoneOrder, list: &[TzRegion], site: &str) -> Vec<TzRegion> {
        regions_by(by, list, tz(site))
            .into_iter()
            .map(|r| r.region)
            .collect()
    }

    fn position(regions: &[TzRegion], region: TzRegion) -> usize {
        regions.iter().position(|r| *r == region).unwrap()
    }

    #[test]
    fn offers_every_region_with_sorted_zones_and_only_etc_utc() {
        let regions = regions_by(Alphabetical, &[], tz("UTC"));
        let names: Vec<_> = regions.iter().map(|r| r.region).collect();
        assert_eq!(
            names,
            [
                Africa, America, Antarctica, Arctic, Asia, Atlantic, Australia, Europe, Indian,
                Pacific, Etc
            ]
        );
        assert_eq!(
            regions.last().map(|r| r.zones.as_slice()),
            Some(&["Etc/UTC"][..])
        );
        for r in &regions {
            assert!(
                r.zones.is_sorted(),
                "zones of {:?} are not sorted by name",
                r.region
            );
        }
    }

    #[test]
    fn filters_regions_and_keeps_etc_even_when_not_listed() {
        assert_eq!(
            order(Alphabetical, &[Europe, Asia], "UTC"),
            [Asia, Europe, Etc]
        );
        assert_eq!(order(Alphabetical, &[Etc], "UTC"), [Etc]);
        assert_eq!(order(Alphabetical, &[], "UTC").len(), 11);
    }

    #[test]
    fn listed_follows_the_order_of_the_list() {
        assert_eq!(
            order(Listed, &[Pacific, Europe, Asia], "UTC"),
            [Pacific, Europe, Asia, Etc]
        );
    }

    #[test]
    fn listed_keeps_etc_last_even_when_listed_first() {
        assert_eq!(order(Listed, &[Etc, Europe], "UTC"), [Europe, Etc]);
    }

    #[test]
    fn listed_without_list_is_alphabetical() {
        assert_eq!(order(Listed, &[], "UTC"), order(Alphabetical, &[], "UTC"));
    }

    #[test]
    fn nearest_puts_the_site_region_first_and_etc_last() {
        let regions = order(Nearest, &[], "Europe/Madrid");
        assert_eq!(regions.first(), Some(&Europe));
        assert_eq!(regions.last(), Some(&Etc));
        assert!(position(&regions, Africa) < position(&regions, Asia));
        assert!(position(&regions, Asia) < position(&regions, Australia));
    }

    // `Pacific` spans from -11 to +14 hours: a linear mean would put its center around +5.7, far
    // from `America`; the circular mean places it next to the date line (-11.5).
    #[test]
    fn nearest_averages_offsets_around_the_clock() {
        assert_eq!(
            order(Nearest, &[Europe, Pacific], "America/Mexico_City"),
            [Pacific, Europe, Etc]
        );
    }

    // From Auckland (+13), `America` (-4.9) is about 6 hours away across the date line; a linear
    // distance would put it almost 18 hours away, farther than `Europe` (+1.5).
    #[test]
    fn nearest_measures_distances_around_the_clock() {
        assert_eq!(
            order(Nearest, &[Europe, America], "Pacific/Auckland"),
            [America, Europe, Etc]
        );
    }

    #[test]
    fn nearest_without_site_region_sorts_every_region_by_distance() {
        assert_eq!(
            order(Nearest, &[Australia, Africa], "Europe/Madrid"),
            [Africa, Australia, Etc]
        );
        // From UTC, `Pacific` (-11.5) is the farthest region, ahead of `Australia` (+10.1).
        let regions = order(Nearest, &[], "UTC");
        assert_eq!(regions[regions.len() - 2..], [Pacific, Etc]);
    }

    #[test]
    fn alias_zones_exist_are_sorted_and_are_not_offered() {
        assert!(ALIAS_ZONES.is_sorted(), "ALIAS_ZONES must be sorted");
        for alias in ALIAS_ZONES {
            assert!(
                alias.parse::<Tz>().is_ok(),
                "{alias} is not a chrono-tz zone"
            );
            assert!(!Timezone::is_supported_in(alias, &[]), "{alias} is offered");
        }
        // Canonical zones and zones merged since 2013 are still offered.
        for zone in ["Asia/Kolkata", "Europe/Kyiv", "Europe/Oslo", "Africa/Accra"] {
            assert!(
                Timezone::is_supported_in(zone, &[]),
                "{zone} is not offered"
            );
        }
        assert!(Timezone::is_supported_in("Asia/Istanbul", &[]));
        assert!(Timezone::is_supported_in("Europe/Nicosia", &[]));
    }
}
