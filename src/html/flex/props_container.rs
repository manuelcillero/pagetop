use crate::{AutoDefault, CowStr};

// **< Behavior >***********************************************************************************

/// Comportamiento de los elementos si no caben en una línea del contenedor [`Flex`].
///
/// Si el contenedor aplica [`align::Gap`] y un [`ItemSize`] porcentual en los hijos, entonces usar
/// [`Behavior::Wrap`] en vez de [`Behavior::NoWrap`] (su valor por defecto) puede provocar saltos
/// de línea prematuros. En la sección "Cómo combinarlo con `Gap`" de `ItemSize` se explica el
/// porqué.
///
/// [`Flex`]: crate::base::component::Flex
/// [`align::Gap`]: crate::html::align::Gap
/// [`ItemSize`]: crate::html::flex::ItemSize
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
    pub(crate) fn value(self) -> CowStr {
        match self {
            Self::NoWrap => "".into(),
            Self::Wrap => "wrap".into(),
            Self::WrapReverse => "wrap-reverse".into(),
        }
    }
}

// **< ContentJustify >*****************************************************************************

/// Alineación de los elementos en el eje principal de un contenedor
/// [`Flex`](crate::base::component::Flex).
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ContentJustify {
    /// Por defecto, el navegador no fuerza ninguna alineación (`justify-content: normal` no
    /// explícito).
    #[default]
    Normal,
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
    pub(crate) fn value(self) -> CowStr {
        match self {
            Self::Normal => "".into(),
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

/// Dirección del eje principal de un contenedor [`Flex`](crate::base::component::Flex).
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
    pub(crate) fn value(self) -> CowStr {
        match self {
            Self::Row => "".into(),
            Self::RowReverse => "row-reverse".into(),
            Self::Column => "column".into(),
            Self::ColumnReverse => "column-reverse".into(),
        }
    }
}
