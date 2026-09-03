//! Enums semánticos que configuran [`Flex`](super::Flex), a nivel de contenedor.

use crate::html::unit::UnitValue;
use crate::{AutoDefault, CowStr};

// **< Align >**************************************************************************************

/// Alineación de los elementos en el eje transversal de un contenedor [`Flex`](super::Flex).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Align {
    /// Por defecto (`align-items: normal` no explícito), mismo efecto que [`Align::Stretch`], salvo
    /// que el elemento tenga su propio tamaño.
    #[default]
    Default,
    /// Alinea los elementos al inicio del eje transversal (`align-items: flex-start`).
    Start,
    /// Alinea los elementos al final del eje transversal (`align-items: flex-end`).
    End,
    /// Centra los elementos en el eje transversal (`align-items: center`).
    Center,
    /// Alinea los elementos por su línea base de texto (`align-items: baseline`).
    Baseline,
    /// Estira los elementos para ocupar todo el eje transversal (`align-items: stretch`).
    Stretch,
}

impl Align {
    // Devuelve el valor CSS de `align-items`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::Start => "flex-start".into(),
            Self::End => "flex-end".into(),
            Self::Center => "center".into(),
            Self::Baseline => "baseline".into(),
            Self::Stretch => "stretch".into(),
        }
    }
}

// **< AlignContent >*******************************************************************************

/// Alineación de varias líneas en un contenedor [`Flex`](super::Flex).
///
/// Sólo tiene efecto si el contenedor usa [`Behavior::Wrap`] o [`Behavior::WrapReverse`] y genera
/// más de una línea de elementos.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum AlignContent {
    /// Por defecto (`align-content: normal` no explícito), como en [`AlignContent::Stretch`], las
    /// líneas se estiran para ocupar el espacio sobrante del eje transversal, sin efecto visible si
    /// el contenedor no tiene ningún espacio sobrante que repartir (p. ej. una altura `auto`
    /// ajustada al contenido).
    #[default]
    Default,
    /// Alinea las líneas al inicio del eje transversal (`align-content: flex-start`).
    Start,
    /// Alinea las líneas al final del eje transversal (`align-content: flex-end`).
    End,
    /// Centra las líneas en el eje transversal (`align-content: center`).
    Center,
    /// Reparte el espacio sobrante entre las líneas (`align-content: space-between`).
    SpaceBetween,
    /// Reparte el espacio sobrante alrededor de cada línea (`align-content: space-around`).
    SpaceAround,
    /// Reparte el espacio sobrante en partes iguales, incluidos los extremos
    /// (`align-content: space-evenly`).
    SpaceEvenly,
    /// Estira las líneas para ocupar todo el eje transversal (`align-content: stretch`).
    Stretch,
}

impl AlignContent {
    // Devuelve el valor CSS de `align-content`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::Start => "flex-start".into(),
            Self::End => "flex-end".into(),
            Self::Center => "center".into(),
            Self::SpaceBetween => "space-between".into(),
            Self::SpaceAround => "space-around".into(),
            Self::SpaceEvenly => "space-evenly".into(),
            Self::Stretch => "stretch".into(),
        }
    }
}

// **< Behavior >***********************************************************************************

/// Comportamiento de los elementos si no caben en una línea del contenedor [`Flex`](super::Flex).
///
/// Si el contenedor aplica [`Gap`] y un [`ItemSize`](super::ItemSize) porcentual en los hijos,
/// entonces usar [`Behavior::Wrap`] en vez de [`Behavior::NoWrap`] (su valor por defecto) puede
/// provocar saltos de línea prematuros. En la sección "Cómo combinarlo con `Gap`" de `ItemSize`
/// se explica el porqué.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Behavior {
    /// Por defecto, no se dividen en varias líneas: se comprimen o desbordan (`flex-wrap: nowrap`
    /// no explícito).
    #[default]
    NoWrap,
    /// Se dividen en varias líneas cuando no caben en una sola (`flex-wrap: wrap`).
    Wrap,
    /// Igual que [`Behavior::Wrap`], pero las líneas se apilan en orden inverso
    /// (`flex-wrap: wrap-reverse`).
    WrapReverse,
}

impl Behavior {
    // Devuelve el valor CSS de `flex-wrap`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::NoWrap => "".into(),
            Self::Wrap => "wrap".into(),
            Self::WrapReverse => "wrap-reverse".into(),
        }
    }
}

// **< ContentJustify >*****************************************************************************

/// Alineación de los elementos en el eje principal de un contenedor [`Flex`](super::Flex).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ContentJustify {
    /// Por defecto, el navegador no fuerza ninguna alineación (`justify-content: normal` no
    /// explícito).
    #[default]
    Default,
    /// Alinea los elementos al inicio del eje principal (`justify-content: flex-start`).
    Start,
    /// Alinea los elementos al final del eje principal (`justify-content: flex-end`).
    End,
    /// Centra los elementos en el eje principal (`justify-content: center`).
    Center,
    /// Reparte el espacio sobrante entre los elementos (`justify-content: space-between`).
    SpaceBetween,
    /// Reparte el espacio sobrante alrededor de cada elemento (`justify-content: space-around`).
    SpaceAround,
    /// Reparte el espacio sobrante en partes iguales, incluidos los extremos
    /// (`justify-content: space-evenly`).
    SpaceEvenly,
}

impl ContentJustify {
    // Devuelve el valor CSS de `justify-content`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::Start => "flex-start".into(),
            Self::End => "flex-end".into(),
            Self::Center => "center".into(),
            Self::SpaceBetween => "space-between".into(),
            Self::SpaceAround => "space-around".into(),
            Self::SpaceEvenly => "space-evenly".into(),
        }
    }
}

// **< Direction >**********************************************************************************

/// Dirección del eje principal de un contenedor [`Flex`](super::Flex).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Por defecto, los elementos se disponen en fila, de izquierda a derecha
    /// (`flex-direction: row` no explícito).
    #[default]
    Row,
    /// Los elementos se disponen en fila, de derecha a izquierda (`flex-direction: row-reverse`).
    RowReverse,
    /// Los elementos se disponen en columna, de arriba abajo (`flex-direction: column`).
    Column,
    /// Los elementos se disponen en columna, de abajo arriba (`flex-direction: column-reverse`).
    ColumnReverse,
}

impl Direction {
    // Devuelve el valor CSS de `flex-direction`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Row => "".into(),
            Self::RowReverse => "row-reverse".into(),
            Self::Column => "column".into(),
            Self::ColumnReverse => "column-reverse".into(),
        }
    }
}

// **< Gap >****************************************************************************************

/// Espaciado entre los elementos de un contenedor [`Flex`](super::Flex).
///
/// Es un valor continuo, no una utilidad predefinida: se resuelve siempre como estilo
/// `gap`/`row-gap`/`column-gap` en línea, igual que el resto de facetas de
/// [`Flex`](super::Flex)/[`FlexItem`](super::FlexItem).
///
/// Si se combina con un [`ItemSize`](super::ItemSize) porcentual sobre los hijos, la sección "Cómo
/// combinarlo con `Gap`" de `ItemSize` explica cómo evitar que el hueco desborde el contenedor.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Gap {
    /// Por defecto, no hay espaciado (`gap: normal` no explícito).
    #[default]
    None,
    /// Mismo espaciado entre filas y columnas.
    Both(UnitValue),
    /// Espaciado distinto entre filas y columnas.
    Distinct { row: UnitValue, column: UnitValue },
}

impl Gap {
    // Declaraciones de estilo (propiedad, valor) para este espaciado; vacío si no hay ninguna
    // medible (`UnitValue::None`/`UnitValue::Auto` no producen ningún estilo).
    pub(super) fn styles(self) -> Vec<(&'static str, CowStr)> {
        match self {
            Self::None => Vec::new(),
            Self::Both(value) => {
                if value.is_measurable() {
                    vec![("gap", value.into())]
                } else {
                    Vec::new()
                }
            }
            Self::Distinct { row, column } => {
                let mut styles = Vec::new();
                if row.is_measurable() {
                    styles.push(("row-gap", row.into()));
                }
                if column.is_measurable() {
                    styles.push(("column-gap", column.into()));
                }
                styles
            }
        }
    }
}
