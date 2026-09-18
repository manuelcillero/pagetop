//! Definiciones de alineación compartidas por los contenedores [`Flex`] y [`Grid`].
//!
//! Reúne las definiciones CSS que Flexbox y CSS Grid resuelven de forma idéntica, con el mismo
//! nombre de propiedad y mismo catálogo de valores. La propia especificación CSS no los considera
//! una característica exclusiva de ninguno de los dos modos, sino del [CSS Box Alignment Module]
//! que ambos comparten:
//!
//! - [`align::Items`] (`align-items`), el contenedor alinea los elementos en el eje transversal
//!   (Flex) o en el eje de filas (Grid).
//! - [`align::Content`] (`align-content`), el contenedor alinea las líneas (Flex) o las pistas
//!   (Grid), si sobra espacio en ese eje.
//! - [`align::Gap`] (`gap`/`row-gap`/`column-gap`), define el espaciado entre elementos (Flex) o
//!   entre pistas (Grid).
//! - [`align::ItemSelf`] (`align-self`), aplicado en el propio ítem para sobrescribir la alineación
//!   del contenedor.
//!
//! # Lo que no vive aquí
//!
//! `justify-items`/`justify-self` no tienen equivalente en Flexbox. Sólo hay un eje principal y uno
//! transversal, intercambiables con [`Direction`], no dos ejes independientes como en Grid.
//! `justify-content` comparte nombre de propiedad en ambos modos, pero su catálogo de valores es
//! distinto porque Grid admite `stretch` y Flex no. Los tres tienen su propio tipo en su propio
//! módulo (ver la documentación de [`grid`](crate::html::grid) para el detalle completo de qué se
//! comparte y qué no).
//!
//! [CSS Box Alignment Module]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_alignment
//! [`Flex`]: crate::base::component::Flex
//! [`Grid`]: crate::base::component::Grid
//! [`align::Items`]: crate::html::align::Items
//! [`align::Content`]: crate::html::align::Content
//! [`align::ItemSelf`]: crate::html::align::ItemSelf
//! [`align::Gap`]: crate::html::align::Gap
//! [`Direction`]: crate::html::flex::Direction

use crate::html::unit::UnitValue;
use crate::{AutoDefault, CowStr};

// **< Items >**************************************************************************************

/// Alinea los elementos en un contenedor [`Flex`] (eje transversal) o [`Grid`] (eje de filas).
///
/// [`Flex`]: crate::base::component::Flex
/// [`Grid`]: crate::base::component::Grid
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Items {
    /// Por defecto (`align-items: normal` no explícito), mismo efecto que [`Items::Stretch`], salvo
    /// que el elemento tenga su propio tamaño.
    #[default]
    Normal,
    /// Alinea los elementos al inicio del eje (`align-items: flex-start`).
    Start,
    /// Alinea los elementos al final del eje (`align-items: flex-end`).
    End,
    /// Centra los elementos en el eje (`align-items: center`).
    Center,
    /// Alinea los elementos por su línea base de texto (`align-items: baseline`).
    Baseline,
    /// Estira los elementos para ocupar todo el eje (`align-items: stretch`).
    Stretch,
}

impl Items {
    // Devuelve el valor CSS de `align-items`, o "" para el valor por defecto.
    pub(crate) fn value(self) -> CowStr {
        match self {
            Self::Normal => "".into(),
            Self::Start => "flex-start".into(),
            Self::End => "flex-end".into(),
            Self::Center => "center".into(),
            Self::Baseline => "baseline".into(),
            Self::Stretch => "stretch".into(),
        }
    }
}

// **< Content >************************************************************************************

/// Alinea varias líneas (en un contenedor [`Flex`]) o varias pistas (en un contenedor [`Grid`]).
///
/// En Flex sólo tiene efecto si el contenedor usa [`Behavior::Wrap`] o [`Behavior::WrapReverse`] y
/// genera más de una línea de elementos.
///
/// En Grid, en cambio, se aplica en el eje de filas, cuando sobra espacio en ese eje y siempre que
/// la suma de las pistas de fila sea menor que la altura del contenedor, sin depender de ningún
/// ajuste de línea.
///
/// [`Flex`]: crate::base::component::Flex
/// [`Grid`]: crate::base::component::Grid
/// [`Behavior::Wrap`]: crate::html::flex::Behavior::Wrap
/// [`Behavior::WrapReverse`]: crate::html::flex::Behavior::WrapReverse
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Content {
    /// Por defecto (`align-content: normal` no explícito), como en [`Content::Stretch`], las líneas
    /// o pistas se estiran para ocupar el espacio sobrante, sin efecto visible si no hay ningún
    /// espacio sobrante que repartir (p. ej. una altura `auto` ajustada al contenido).
    #[default]
    Normal,
    /// Alinea al inicio del eje (`align-content: flex-start`).
    Start,
    /// Alinea al final del eje (`align-content: flex-end`).
    End,
    /// Centra en el eje (`align-content: center`).
    Center,
    /// Reparte el espacio sobrante entre las líneas o pistas (`align-content: space-between`).
    SpaceBetween,
    /// Reparte el espacio sobrante alrededor de cada línea o pista (`align-content: space-around`).
    SpaceAround,
    /// Reparte el espacio sobrante en partes iguales, incluidos los extremos
    /// (`align-content: space-evenly`).
    SpaceEvenly,
    /// Estira las líneas o pistas para ocupar todo el eje (`align-content: stretch`).
    Stretch,
}

impl Content {
    // Devuelve el valor CSS de `align-content`, o "" para el valor por defecto.
    pub(crate) fn value(self) -> CowStr {
        match self {
            Self::Normal => "".into(),
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

// **< Gap >****************************************************************************************

/// Espaciado entre los elementos ([`Flex`]) o entre las pistas ([`Grid`]) de un contenedor.
///
/// Se resuelve como estilo `gap`/`row-gap`/`column-gap` según aplique.
///
/// En Flex, si se va a combinar con un [`ItemSize`] porcentual sobre los hijos, la sección "Cómo
/// combinarlo con `Gap`" de [`ItemSize`] explica cómo evitar que el hueco desborde el contenedor.
/// Esa combinación no aplica a Grid, donde el reparto de espacio entre pistas no tiene el mismo
/// problema.
///
/// [`Flex`]: crate::base::component::Flex
/// [`Grid`]: crate::base::component::Grid
/// [`ItemSize`]: crate::html::flex::ItemSize
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
    // Declaraciones de estilo (propiedad, valor) para este espaciado; cada hueco a `None` si no hay
    // ninguna medible (`UnitValue::None`/`UnitValue::Auto` no producen ningún estilo).
    pub(crate) fn styles(self) -> [Option<(&'static str, CowStr)>; 2] {
        match self {
            Self::None => [None, None],
            Self::Both(value) => [Self::style("gap", value), None],
            Self::Distinct { row, column } => [
                Self::style("row-gap", row),
                Self::style("column-gap", column),
            ],
        }
    }

    // Declaración (propiedad, valor) para un valor medible, o `None` si no lo es.
    fn style(property: &'static str, value: UnitValue) -> Option<(&'static str, CowStr)> {
        value.is_measurable().then(|| (property, value.into()))
    }
}

// **< ItemSelf >***********************************************************************************

/// Alineación individual de [`FlexItem`] (eje transversal) o [`GridItem`] (eje de filas).
///
/// Sobrescribe la alineación del contenedor con el valor de `align-self`. En Grid, `flex-start`/
/// `flex-end` equivalen a `start`/`end`.
///
/// [`FlexItem`]: crate::html::flex::FlexItem
/// [`GridItem`]: crate::html::grid::GridItem
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemSelf {
    /// Por defecto, hereda la alineación del contenedor (`align-self: auto` no explícito).
    #[default]
    Default,
    /// Alinea el ítem al inicio del eje (`align-self: flex-start`).
    Start,
    /// Alinea el ítem al final del eje (`align-self: flex-end`).
    End,
    /// Centra el ítem en el eje (`align-self: center`).
    Center,
    /// Alinea el ítem por su línea base de texto (`align-self: baseline`).
    Baseline,
    /// Estira el ítem para ocupar todo el eje (`align-self: stretch`).
    Stretch,
}

impl ItemSelf {
    // Devuelve el valor CSS de `align-self`, o "" para el valor por defecto.
    pub(crate) fn value(self) -> CowStr {
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
