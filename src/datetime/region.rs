use crate::locale::Lc;

/// Regiones de la base de datos de zonas horarias de la IANA.
///
/// Cada región es el nombre que antecede la primera `/` del identificador de una zona horaria (p.
/// ej. `Europe` en `"Europe/Madrid"`). [`Timezone::supported_by_region()`] agrupa por ellas las
/// zonas horarias que se ofrecen para elegir, y [`form::SelectTimezone`] permite limitarlas.
///
/// `TzRegion::Etc` sólo contiene `Etc/UTC`, equivalente a `UTC`, la zona horaria de respaldo, y se
/// ofrece siempre.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// assert_eq!(TzRegion::Europe.as_str(), "Europe");
/// assert_eq!(TzRegion::from_name("europe"), Some(TzRegion::Europe));
/// assert_eq!(TzRegion::from_name("US"), None);
/// ```
///
/// [`Timezone::supported_by_region()`]: super::Timezone::supported_by_region
/// [`form::SelectTimezone`]: crate::base::component::form::SelectTimezone
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TzRegion {
    /// África.
    Africa,
    /// América.
    America,
    /// Antártida.
    Antarctica,
    /// Ártico.
    Arctic,
    /// Asia.
    Asia,
    /// Océano Atlántico.
    Atlantic,
    /// Australia.
    Australia,
    /// Otras: sólo `Etc/UTC`.
    Etc,
    /// Europa.
    Europe,
    /// Océano Índico.
    Indian,
    /// Océano Pacífico.
    Pacific,
}

impl TzRegion {
    /// Devuelve el nombre IANA de la región (p. ej. `"Europe"`).
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Africa => "Africa",
            Self::America => "America",
            Self::Antarctica => "Antarctica",
            Self::Arctic => "Arctic",
            Self::Asia => "Asia",
            Self::Atlantic => "Atlantic",
            Self::Australia => "Australia",
            Self::Etc => "Etc",
            Self::Europe => "Europe",
            Self::Indian => "Indian",
            Self::Pacific => "Pacific",
        }
    }

    /// Devuelve la región con el nombre IANA dado, sin distinguir mayúsculas ni tener en cuenta los
    /// espacios del principio y del final; o `None` si no es ninguna de las regiones disponibles
    /// (las que sólo agrupan alias heredados, como `"US"` o `"Canada"`, no lo son).
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim();
        [
            Self::Africa,
            Self::America,
            Self::Antarctica,
            Self::Arctic,
            Self::Asia,
            Self::Atlantic,
            Self::Australia,
            Self::Etc,
            Self::Europe,
            Self::Indian,
            Self::Pacific,
        ]
        .into_iter()
        .find(|region| region.as_str().eq_ignore_ascii_case(name))
    }

    /// Devuelve el nombre traducido de la región (p. ej. *"Europa"* en español).
    pub fn label(&self) -> Lc {
        Lc::l(match self {
            Self::Africa => "timezone_region_africa",
            Self::America => "timezone_region_america",
            Self::Antarctica => "timezone_region_antarctica",
            Self::Arctic => "timezone_region_arctic",
            Self::Asia => "timezone_region_asia",
            Self::Atlantic => "timezone_region_atlantic",
            Self::Australia => "timezone_region_australia",
            Self::Etc => "timezone_region_etc",
            Self::Europe => "timezone_region_europe",
            Self::Indian => "timezone_region_indian",
            Self::Pacific => "timezone_region_pacific",
        })
    }
}
