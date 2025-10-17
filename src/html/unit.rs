use crate::AutoDefault;

use serde::{Deserialize, Deserializer};

use std::fmt;
use std::str::FromStr;

/// Representa una **unidad CSS** lista para formatear o deserializar.
///
/// ## Unidades soportadas
///
/// - **Absolutas** *(valores enteros, `isize`)*:
///   - `Cm(isize)` - `cm` (centímetros)
///   - `In(isize)` - `in` (pulgadas; `1in = 96px = 2.54cm`)
///   - `Mm(isize)` - `mm` (milímetros)
///   - `Pc(isize)` - `pc` (picas; `1pc = 12pt`)
///   - `Pt(isize)` - `pt` (puntos; `1pt = 1/72in`)
///   - `Px(isize)` - `px` (píxeles; `1px = 1/96in`)
///
/// - **Relativas** *(valores decimales, `f32`)*:
///   - `RelEm(f32)`  - `em`  (relativa al tamaño de fuente del elemento)
///   - `RelRem(f32)` - `rem` (relativa al tamaño de fuente de `:root`)
///   - `RelPct(f32)` - `%`   (porcentaje relativo al elemento padre)
///   - `RelVh(f32)`  - `vh`  (1% de la **altura** del viewport)
///   - `RelVw(f32)`  - `vw`  (1% del **ancho** del viewport)
///
/// ## Valores especiales
///
/// - `None` - equivale a un texto vacío (`""`), útil para atributos opcionales.
/// - `Auto` - equivale a `"auto"`.
/// - `Zero` - equivale a `"0"` (cero sin unidad).
///
/// ## Características
///
/// - Soporta unidades **absolutas** (`cm`, `in`, `mm`, `pc`, `pt`, `px`) y **relativas** (`em`,
///   `rem`, `%`, `vh`, `vw`).
/// - `FromStr` para convertir desde texto (p. ej., `"12px"`, `"1.25rem"`, `"auto"`).
/// - `Display` para formatear a cadena (p. ej., `UnitValue::Px(12)` genera `"12px"`).
/// - `Deserialize` delega en `FromStr`, garantizando una gramática única.
///
/// ## Ejemplos
///
/// ```rust
/// # use pagetop::prelude::*;
/// use std::str::FromStr;
///
/// assert_eq!(UnitValue::from_str("16px").unwrap(), UnitValue::Px(16));
/// assert_eq!(UnitValue::from_str("1.25rem").unwrap(), UnitValue::RelRem(1.25));
/// assert_eq!(UnitValue::from_str("33%").unwrap(), UnitValue::RelPct(33.0));
/// assert_eq!(UnitValue::from_str("auto").unwrap(), UnitValue::Auto);
/// assert_eq!(UnitValue::from_str("").unwrap(), UnitValue::None);
/// assert_eq!(UnitValue::from_str("0").unwrap(), UnitValue::Zero);
/// ```
///
/// ## Notas
///
/// - Las absolutas **no aceptan** decimales (p. ej., `"1.5px"` sería erróneo).
/// - Se aceptan signos `+`/`-` en todas las unidades (p. ej., `"-12px"`, `"+0.5em"`).
/// - La comparación de unidad es *case-insensitive* al interpretar el texto (`"PX"`, `"Px"`, …).
/// - **Sobre píxeles**: Los píxeles (px) son relativos al dispositivo de visualización. En
///   dispositivos con baja densidad de píxeles (dpi), 1px equivale a un píxel (punto) del
///   dispositivo. En impresoras y pantallas de alta resolución, 1px implica múltiples píxeles del
///   dispositivo.
/// - **Sobre `em` y `rem`**:
///   - `em` es **relativo al tamaño de fuente *del propio elemento***. Si el elemento hereda o
///     cambia su `font-size`, todos los valores en `em` dentro de él **se escalan en cascada**.
///   - `rem` es **relativo al tamaño de fuente del elemento raíz** (`:root`/`html`), **no se verá
///     afectado** por cambios de `font-size` en elementos anidados.
///   - Ejemplo: si `:root { font-size: 16px }` y un contenedor tiene `font-size: 20px`, entonces
///     dentro del contenedor `1em == 20px` pero `1rem == 16px`.
///   - Uso típico: `rem` para tipografía y espaciados globales (consistencia al cambiar la base del
///     sitio); `em` para tamaños que deban escalar **con el propio componente** (p. ej.,
///     `padding: 0.5em` que crece si el componente aumenta su `font-size`).
/// - **Sobre el viewport**: Si el ancho de la ventana del navegador es de 50cm, 1vw equivale a
///   0.5cm (1vw siempre es 1% del ancho del viewport, independientemente del zoom del navegador o
///   la densidad de píxeles del dispositivo).
#[rustfmt::skip]
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum UnitValue {
    #[default]
    None,
    Auto,
    /// Cero sin unidad.
    Zero,
    /// Centímetros.
    Cm(isize),
    /// Pulgadas (1in = 96px = 2.54cm).
    In(isize),
    /// Milímetros.
    Mm(isize),
    /// Picas (1pc = 12pt).
    Pc(isize),
    /// Puntos (1pt = 1/72in).
    Pt(isize),
    /// Píxeles (1px = 1/96in).
    Px(isize),
    /// Relativo al tamaño de la fuente del elemento.
    RelEm(f32),
    /// Relativo al tamaño de la fuente del elemento raíz.
    RelRem(f32),
    /// Porcentaje relativo al elemento padre.
    RelPct(f32),
    /// Relativo al 1% de la altura del viewport.
    RelVh(f32),
    /// Relativo al 1% del ancho del viewport.
    RelVw(f32),
}

impl UnitValue {
    /// Indica si el valor es **medible**, incluyendo `Zero` sin unidad.
    ///
    /// Devuelve `false` para [`UnitValue::None`] y [`UnitValue::Auto`].
    ///
    /// # Ejemplos
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// // Numéricos (incluido el cero sin unidad).
    /// assert!(UnitValue::Zero.is_measurable());
    /// assert!(UnitValue::Px(0).is_measurable());
    /// assert!(UnitValue::Px(10).is_measurable());
    /// assert!(UnitValue::RelPct(33.0).is_measurable());
    /// // No numéricos.
    /// assert!(!UnitValue::None.is_measurable());
    /// assert!(!UnitValue::Auto.is_measurable());
    /// ```
    #[inline]
    pub const fn is_measurable(&self) -> bool {
        !matches!(self, UnitValue::None | UnitValue::Auto)
    }
}

/// Formatea la unidad como cadena CSS.
///
/// Reglas:
///
/// - `None` - `""` (cadena vacía).
/// - `Auto` - `"auto"`.
/// - `Zero` - `"0"` (cero sin unidad).
/// - Absolutas - entero con su unidad: `Px(12)` a `"12px"`.
/// - Relativas - número en punto flotante; si es entero, se imprime sin decimales:
///   - `RelEm(2.0)` a `"2em"`
///   - `RelPct(33.5)` a `"33.5%"`
#[rustfmt::skip]
impl fmt::Display for UnitValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitValue::None       => write!(f, ""),
            UnitValue::Auto       => write!(f, "auto"),
            UnitValue::Zero       => write!(f, "0"),
            // Valor absoluto.
            UnitValue::Cm(av)     => write!(f, "{av}cm"),
            UnitValue::In(av)     => write!(f, "{av}in"),
            UnitValue::Mm(av)     => write!(f, "{av}mm"),
            UnitValue::Pc(av)     => write!(f, "{av}pc"),
            UnitValue::Pt(av)     => write!(f, "{av}pt"),
            UnitValue::Px(av)     => write!(f, "{av}px"),
            // Valor relativo.
            UnitValue::RelEm(rv)  => write!(f, "{rv}em"),
            UnitValue::RelRem(rv) => write!(f, "{rv}rem"),
            UnitValue::RelPct(rv) => write!(f, "{rv}%"),
            UnitValue::RelVh(rv)  => write!(f, "{rv}vh"),
            UnitValue::RelVw(rv)  => write!(f, "{rv}vw"),
        }
    }
}

/// Convierte una cadena a [`UnitValue`] siguiendo una gramática CSS acotada.
///
/// ## Acepta
///
/// - `""` para `UnitValue::None`
/// - `"auto"`
/// - **Cero sin unidad**: `"0"`, `"+0"`, `"-0"`, `"0.0"`, `"0."`, `".0"` para `UnitValue::Zero`
/// - Porcentaje: `"<n>%"` (p. ej., `"33%"`, `"33 %"`)
/// - Absolutas enteras: `"<entero><unidad>"`, p. ej., `"12px"`, `"-5pt"`
/// - Relativas decimales: `"<float><unidad>"`, p. ej., `"1.25rem"`, `"-0.5vh"`, `".5em"`, `"1.rem"`
///
/// (Se toleran espacios entre número y unidad: `"12 px"`, `"1.5  rem"`).
///
/// ## Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// use std::str::FromStr;
///
/// assert_eq!(UnitValue::from_str("12px").unwrap(), UnitValue::Px(12));
/// assert!(UnitValue::from_str("12").is_err());
/// ```
///
/// ## Errores de interpretación
///
/// - Falta la unidad cuando es necesaria (p. ej., `"12"`, excepto para el valor cero).
/// - Decimales en valores que deben ser absolutos (p. ej. `"1.5px"`).
/// - Unidades desconocidas (p. ej., `"10ch"`, no soportada aún).
/// - Notación científica o bases no decimales: `"1e3vw"`, `"0x10px"` (no soportadas). Los ceros a
///   la izquierda (p. ej. `"020px"`) se interpretan en **base 10** (`20px`).
///
/// La comparación de la unidad es *case-insensitive*.
impl FromStr for UnitValue {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let s = input.trim();
        if s.is_empty() {
            return Ok(UnitValue::None);
        }
        if s.eq_ignore_ascii_case("auto") {
            return Ok(UnitValue::Auto);
        }

        match s.find(|c: char| c.is_ascii_alphabetic() || c == '%') {
            None => {
                let n: f32 = s
                    .parse()
                    .map_err(|e| format!("Invalid number `{s}`: {e}"))?;
                if n == 0.0 {
                    Ok(UnitValue::Zero)
                } else {
                    Err(
                        "Missing unit (expected one of cm,in,mm,pc,pt,px,em,rem,vh,vw, or %)"
                            .to_string(),
                    )
                }
            }
            Some(split_pos) => {
                let (num_str, unit_str) = s.split_at(split_pos);
                let u = unit_str.trim();
                let n = num_str.trim();

                let parse_abs = |n_s: &str| -> Result<isize, String> {
                    n_s.parse::<isize>()
                        .map_err(|e| format!("Invalid integer `{n_s}`: {e}"))
                };
                let parse_rel = |n_s: &str| -> Result<f32, String> {
                    n_s.parse::<f32>()
                        .map_err(|e| format!("Invalid float `{n_s}`: {e}"))
                };

                match u.to_ascii_lowercase().as_str() {
                    // Unidades absolutas.
                    "cm" => Ok(UnitValue::Cm(parse_abs(n)?)),
                    "in" => Ok(UnitValue::In(parse_abs(n)?)),
                    "mm" => Ok(UnitValue::Mm(parse_abs(n)?)),
                    "pc" => Ok(UnitValue::Pc(parse_abs(n)?)),
                    "pt" => Ok(UnitValue::Pt(parse_abs(n)?)),
                    "px" => Ok(UnitValue::Px(parse_abs(n)?)),
                    // Unidades relativas.
                    "em" => Ok(UnitValue::RelEm(parse_rel(n)?)),
                    "rem" => Ok(UnitValue::RelRem(parse_rel(n)?)),
                    "vh" => Ok(UnitValue::RelVh(parse_rel(n)?)),
                    "vw" => Ok(UnitValue::RelVw(parse_rel(n)?)),
                    // Porcentaje como unidad.
                    "%" => Ok(UnitValue::RelPct(parse_rel(n)?)),
                    // Unidad desconocida.
                    _ => Err(format!("Unknown unit: `{u}`")),
                }
            }
        }
    }
}

/// Deserializa desde una cadena usando la misma gramática que [`FromStr`].
///
/// ### Ejemplo con `serde_json`
/// ```rust
/// # use pagetop::prelude::*;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Style { width: UnitValue }
///
/// // "{\"width\":\"12px\"}" deserializa como `Style { width: UnitValue::Px(12) }`
/// ```
impl<'de> Deserialize<'de> for UnitValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        raw.parse().map_err(serde::de::Error::custom)
    }
}
