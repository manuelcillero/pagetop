use crate::AutoDefault;

use regex::Regex;
use serde::{Deserialize, Deserializer};

use std::fmt;

// Sobre píxeles: Los píxeles (px) son relativos al dispositivo de visualización. En dispositivos
// con baja densidad de píxeles (dpi), 1px equivale a un píxel (punto) del dispositivo. En
// impresoras y pantallas de alta resolución, 1px implica múltiples píxeles del dispositivo.

// Sobre em: 2em significa 2 veces el tamaño de la fuente actual. Las unidades em y rem son muy
// útiles para crear diseños completamente escalables.

// Sobre el viewport: Si el ancho de la ventana del navegador es de 50cm, 1vw equivale a 0.5cm.

#[rustfmt::skip]
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Value {
    #[default]
    None,
    Auto,

    Cm(isize),    // Centímetros.
    In(isize),    // Pulgadas (1in = 96px = 2.54cm).
    Mm(isize),    // Milímetros.
    Pc(isize),    // Picas (1pc = 12pt).
    Pt(isize),    // Puntos (1pt = 1/72 of 1in).
    Px(isize),    // Píxeles (1px = 1/96th of 1in).

    RelEm(f32),   // Relativo al tamaño de la fuente del elemento.
    RelRem(f32),  // Relativo al tamaño de la fuente del elemento raíz.
    RelPct(f32),  // Porcentaje relativo al elemento padre.
    RelVh(f32),   // Relativo al 1% de la altura del viewport.
    RelVw(f32),   // Relativo al 1% del valor del viewport.
}

#[rustfmt::skip]
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::None       => write!(f, ""),
            Value::Auto       => write!(f, "auto"),
            // Valor absoluto.
            Value::Cm(av)     => write!(f, "{av}cm"),
            Value::In(av)     => write!(f, "{av}in"),
            Value::Mm(av)     => write!(f, "{av}mm"),
            Value::Pc(av)     => write!(f, "{av}pc"),
            Value::Pt(av)     => write!(f, "{av}pt"),
            Value::Px(av)     => write!(f, "{av}px"),
            // Valor relativo.
            Value::RelEm(rv)  => write!(f, "{rv}em"),
            Value::RelRem(rv) => write!(f, "{rv}rem"),
            Value::RelPct(rv) => write!(f, "{rv}%"),
            Value::RelVh(rv)  => write!(f, "{rv}vh"),
            Value::RelVw(rv)  => write!(f, "{rv}vw"),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        let rex = Regex::new(r"^(auto|\d+(\.\d+)?(cm|in|mm|pc|pt|px|em|rem|%|vh|vw)?)$").unwrap();

        if raw == "auto" {
            return Ok(Value::Auto);
        } else if let Some(captures) = rex.captures(&raw) {
            if let Some(unit) = captures.get(3) {
                let num_str = &raw[..raw.len() - unit.as_str().len()];

                // Analizar como `isize` para unidades absolutas.
                if matches!(unit.as_str(), "cm" | "in" | "mm" | "pc" | "pt" | "px") {
                    let num: isize = num_str.parse().map_err(serde::de::Error::custom)?;
                    return match unit.as_str() {
                        "cm" => Ok(Value::Cm(num)),
                        "in" => Ok(Value::In(num)),
                        "mm" => Ok(Value::Mm(num)),
                        "pc" => Ok(Value::Pc(num)),
                        "pt" => Ok(Value::Pt(num)),
                        "px" => Ok(Value::Px(num)),
                        _ => unreachable!(),
                    };
                }

                // Analizar como `f32` para unidades relativas.
                let num: f32 = num_str.parse().map_err(serde::de::Error::custom)?;
                return match unit.as_str() {
                    "em" => Ok(Value::RelEm(num)),
                    "rem" => Ok(Value::RelRem(num)),
                    "%" => Ok(Value::RelPct(num)),
                    "vh" => Ok(Value::RelVh(num)),
                    "vw" => Ok(Value::RelVw(num)),
                    _ => unreachable!(),
                };
            }
        }
        Err(serde::de::Error::custom(format!(
            "Invalid format for Value: {}",
            raw
        )))
    }
}
