use crate::{global, trace, util};

use super::{TZ_VARIANTS, Tz};

use std::collections::BTreeMap;
use std::sync::LazyLock;

// Identificador de zona horaria configurado para la aplicación, si es válido.
static CONFIG_TZ: LazyLock<Option<Tz>> = LazyLock::new(|| {
    global::SETTINGS
        .app
        .timezone
        .as_deref()
        .and_then(util::non_blank)
        .and_then(|raw| raw.parse().ok())
});

// Zona horaria de respaldo, garantizada incluso sin configuración válida.
const FALLBACK_TZ: Tz = Tz::UTC;

// Regiones de la base IANA que sólo contienen alias heredados (fichero `backward`), todos con una
// zona canónica equivalente en otra región (p. ej. `US/Eastern` es `America/New_York`).
const LEGACY_REGIONS: [&str; 5] = ["Brazil", "Canada", "Chile", "Mexico", "US"];

// Zonas horarias IANA agrupadas por región (lo anterior a la primera `/`), ordenadas por región y
// nombre. Se descartan los nombres sin región (alias heredados como `GB`, `Japan` o `EST5EDT`) y
// las regiones de `LEGACY_REGIONS`. De `Etc` sólo se conserva `Etc/UTC`, cuyo grupo va al final:
// el resto son zonas de desfase fijo (`Etc/GMT+1`...) que no representan ningún lugar. Siguen
// apareciendo los alias heredados que viven dentro de una región normal (p. ej. `Asia/Calcutta`
// junto a `Asia/Kolkata`): `chrono-tz` no distingue zonas canónicas de enlaces y filtrarlos
// exigiría mantener a mano una lista de casi 180 nombres.
static TZ_BY_REGION: LazyLock<Vec<(&'static str, Vec<&'static str>)>> = LazyLock::new(|| {
    let mut regions: BTreeMap<&'static str, Vec<&'static str>> = BTreeMap::new();
    for tz in TZ_VARIANTS.iter() {
        let name = tz.name();
        let Some((region, _)) = name.split_once('/') else {
            continue;
        };
        if LEGACY_REGIONS.contains(&region) || (region == "Etc" && name != "Etc/UTC") {
            continue;
        }
        regions.entry(region).or_default().push(name);
    }
    for names in regions.values_mut() {
        names.sort_unstable();
    }
    let etc = regions.remove_entry("Etc");
    regions.into_iter().chain(etc).collect()
});

/// Zona horaria configurada para la aplicación.
///
/// Resuelve [`global::SETTINGS.app.timezone`](crate::global::App::timezone) contra la base IANA de
/// zonas horarias. Si no se ha configurado o el valor no es válido, se aplica la zona horaria de
/// respaldo (`UTC`).
pub struct Timezone;

impl Timezone {
    /// Inicializa la zona horaria por defecto que utilizará la aplicación.
    ///
    /// Debe llamarse durante la inicialización para indicar si la zona horaria por defecto procede
    /// de la configuración, de una configuración no válida o de la zona horaria de respaldo.
    pub(crate) fn init() {
        match global::SETTINGS
            .app
            .timezone
            .as_deref()
            .and_then(util::non_blank)
        {
            Some(raw) => {
                if let Some(tz) = *CONFIG_TZ {
                    trace::debug!("Default timezone \"{tz}\" (from config: \"{raw}\")");
                } else {
                    trace::warn!(
                        "Default timezone \"{FALLBACK_TZ}\" (fallback, invalid config: \"{raw}\")"
                    );
                }
            }
            _ => trace::debug!("Default timezone \"{FALLBACK_TZ}\" (fallback, no config)"),
        }
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
    /// Cada grupo es la región (lo anterior a la primera `/`, p. ej. `"Europe"`) con los nombres
    /// completos de sus zonas (p. ej. `"Europe/Madrid"`), ordenados por región y nombre; el grupo
    /// `"Etc"`, sólo con `"Etc/UTC"`, va al final. Se excluyen los nombres sin región (`"UTC"`,
    /// `"Japan"`...), las regiones formadas sólo por alias heredados (`"US"`, `"Canada"`...) y las
    /// zonas de desfase fijo (`"Etc/GMT+1"`...). Es la lista que ofrece
    /// [`form::SelectTimezone`](crate::base::component::form::SelectTimezone), útil también para
    /// validar el valor recibido.
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
    pub fn supported_by_region() -> &'static [(&'static str, Vec<&'static str>)] {
        &TZ_BY_REGION
    }
}
