use crate::html::unit::UnitValue;
use crate::{AutoDefault, CowStr, util};

// **< AutoFlow >***********************************************************************************

/// Mecanismo de colocación automática de los elementos en un contenedor [`Grid`].
///
/// Aplica en aquellos elementos que no tienen una posición explícita ([`ItemPlacement`]) en el
/// contenedor.
///
/// [`Grid`]: crate::base::component::Grid
/// [`ItemPlacement`]: super::ItemPlacement
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum AutoFlow {
    /// Por defecto, coloca automáticamente rellenando filas (`grid-auto-flow: row` no explícito).
    #[default]
    Row,
    /// Coloca automáticamente rellenando columnas (`grid-auto-flow: column`).
    Column,
    /// Como `Row`, pero rellena huecos anteriores si un elemento más pequeño cabe en ellos
    /// (`grid-auto-flow: row dense`). Puede alterar el orden visual respecto al del documento con
    /// la misma cautela de accesibilidad que ya documenta [`ItemOrder`] en Flex.
    ///
    /// [`ItemOrder`]: crate::html::flex::ItemOrder
    RowDense,
    /// Como `Column`, con el mismo relleno de huecos (`grid-auto-flow: column dense`).
    ColumnDense,
}

impl AutoFlow {
    // Devuelve el valor CSS de `grid-auto-flow`, o "" para el valor por defecto.
    pub(crate) fn value(self) -> CowStr {
        match self {
            Self::Row => "".into(),
            Self::Column => "column".into(),
            Self::RowDense => "row dense".into(),
            Self::ColumnDense => "column dense".into(),
        }
    }
}

// **< DefaultJustify >*****************************************************************************

/// Alineación de los elementos en el eje de columnas (en línea) de un contenedor [`Grid`].
///
/// Es el valor por defecto que hereda cualquier elemento siempre que [`ItemJustify`] no lo
/// sobrescriba.
///
/// Es al eje de columnas lo que [`align::Items`] es al eje transversal de [`Flex`], pero Grid, a
/// diferencia de Flexbox, sí tiene una propiedad CSS propia para ello (`justify-items`), porque en
/// Grid ningún eje se resuelve "girando" al otro.
///
/// [`align::Items`]: crate::html::align::Items
/// [`Flex`]: crate::base::component::Flex
/// [`Grid`]: crate::base::component::Grid
/// [`ItemJustify`]: super::ItemJustify
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum DefaultJustify {
    /// Por defecto, el navegador no fuerza ninguna alineación (`justify-items: normal` no
    /// explícito, equivalente a `stretch` salvo que el elemento tenga su propio tamaño).
    #[default]
    Normal,
    /// Alinea los elementos al inicio del eje de columnas (`justify-items: start`).
    Start,
    /// Alinea los elementos al final del eje de columnas (`justify-items: end`).
    End,
    /// Centra los elementos en el eje de columnas (`justify-items: center`).
    Center,
    /// Estira los elementos para ocupar toda su columna (`justify-items: stretch`).
    Stretch,
}

impl DefaultJustify {
    // Devuelve el valor CSS de `justify-items`, o "" para el valor por defecto.
    pub(crate) fn value(self) -> CowStr {
        match self {
            Self::Normal => "".into(),
            Self::Start => "start".into(),
            Self::End => "end".into(),
            Self::Center => "center".into(),
            Self::Stretch => "stretch".into(),
        }
    }
}

// **< ContentJustify >*****************************************************************************

/// Alineación de las pistas en el eje de columnas de un contenedor [`Grid`].
///
/// Aplica cuando su tamaño total es menor que el del propio contenedor. No se reutiliza
/// [`flex::ContentJustify`] porque Grid admite `stretch` (estira las pistas para repartir el
/// espacio sobrante entre ellas), un valor sin efecto en Flexbox.
///
/// [`Grid`]: crate::base::component::Grid
/// [`flex::ContentJustify`]: crate::html::flex::ContentJustify
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ContentJustify {
    /// Por defecto, el navegador no fuerza ninguna alineación (`justify-content: normal` no
    /// explícito).
    #[default]
    Normal,
    /// Alinea las pistas al inicio del eje de columnas (`justify-content: start`).
    Start,
    /// Alinea las pistas al final del eje de columnas (`justify-content: end`).
    End,
    /// Centra las pistas en el eje de columnas (`justify-content: center`).
    Center,
    /// Estira las pistas para repartir el espacio sobrante entre ellas
    /// (`justify-content: stretch`).
    Stretch,
    /// Reparte el espacio sobrante entre las pistas (`justify-content: space-between`).
    SpaceBetween,
    /// Reparte el espacio sobrante alrededor de cada pista (`justify-content: space-around`).
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
            Self::Start => "start".into(),
            Self::End => "end".into(),
            Self::Center => "center".into(),
            Self::Stretch => "stretch".into(),
            Self::SpaceBetween => "space-between".into(),
            Self::SpaceAround => "space-around".into(),
            Self::SpaceEvenly => "space-evenly".into(),
        }
    }
}

// **< AxisTrack >**********************************************************************************

/// Anchura o altura de una única pista en un contenedor [`Grid`].
///
/// Se aplica dentro de una lista [`Tracks`], o vía [`Grid::with_columns()`]/[`Grid::with_rows()`]
/// (la rejilla explícita); o suelto, vía [`Grid::with_auto_columns()`]/[`Grid::with_auto_rows()`]
/// (el tamaño de las pistas que el navegador genera automáticamente fuera de esa rejilla).
///
/// [`Grid`]: crate::base::component::Grid
/// [`Grid::with_columns()`]: crate::base::component::Grid::with_columns
/// [`Grid::with_rows()`]: crate::base::component::Grid::with_rows
/// [`Grid::with_auto_columns()`]: crate::base::component::Grid::with_auto_columns
/// [`Grid::with_auto_rows()`]: crate::base::component::Grid::with_auto_rows
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum AxisTrack {
    /// Por defecto, ajusta al contenido repartiendo el espacio sobrante con las demás pistas `Auto`
    /// (`auto`).
    #[default]
    Auto,
    /// Medida fija o porcentual (p. ej. `AxisTrack::Fixed(UnitValue::Px(200))` o
    /// `AxisTrack::Fixed(UnitValue::RelPct(50.0))`).
    Fixed(UnitValue),
    /// Fracción del espacio sobrante repartido entre todas las pistas `Fraction` del mismo
    /// contenedor (`<n>fr`). Es la unidad idiomática de Grid, sin equivalente en [`UnitValue`]
    /// porque no es una medida absoluta ni relativa a nada fuera del propio reparto de Grid.
    Fraction(f32),
    /// El tamaño más pequeño en el que el contenido cabe sin desbordar. El texto se ajusta de línea
    /// todo lo posible, así que el resultado es el ancho de su elemento indivisible más largo
    /// (`min-content`).
    MinContent,
    /// El tamaño que ocuparía el contenido si no tuviera que ajustarse de línea en absoluto, siendo
    /// su ancho natural, sin envolver (`max-content`).
    MaxContent,
}

impl AxisTrack {
    // Devuelve el valor CSS de esta pista. A diferencia del resto de propiedades de Grid/GridItem,
    // un `AxisTrack::Auto` (el valor por defecto del enum) sí produce un valor no vacío ("auto").
    // En vez de omitir la propiedad entera, una lista de pistas necesita un valor por cada hueco
    // para que el número de pistas declaradas sea el correcto.
    pub(crate) fn value(self) -> CowStr {
        match self {
            Self::Auto => "auto".into(),
            Self::Fixed(unit) => unit.value(),
            Self::Fraction(n) => util::join!(n.to_string(), "fr").into(),
            Self::MinContent => "min-content".into(),
            Self::MaxContent => "max-content".into(),
        }
    }
}

// **< Tracks >*************************************************************************************

/// Número máximo de pistas que admite [`Tracks`].
///
/// Ver la nota de la propia `Tracks` sobre por qué es una capacidad fija y no un `Vec`.
pub const MAX_TRACKS: usize = 12;

/// Lista ordenada de pistas para la rejilla explícita de un contenedor [`Grid`].
///
/// Sólo se aplica para [`Grid::with_columns()`] o [`Grid::with_rows()`].
///
/// [`Grid`]: crate::base::component::Grid
/// [`Grid::with_columns()`]: crate::base::component::Grid::with_columns
/// [`Grid::with_rows()`]: crate::base::component::Grid::with_rows
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Tracks {
    // Es un array de capacidad fija (`[Option<AxisTrack>; MAX_TRACKS]`), no un `Vec<AxisTrack>`,
    // porque [`Responsive<T>`](crate::core::theme::Responsive) exige `T: Copy` y un `Vec` no lo es;
    // de lo contrario `Tracks` no podría variar por punto de corte (p. ej. una columna en móvil,
    // tres en escritorio), que es precisamente el caso de uso más habitual de Grid con breakpoints.
    // Doce pistas es más que suficiente para cualquier rejilla real (coincide, no por casualidad,
    // con la tradición de 12 columnas de Bootstrap y otros frameworks CSS).
    tracks: [Option<AxisTrack>; MAX_TRACKS],
}

impl Tracks {
    /// Crea una lista de pistas vacía.
    pub fn new() -> Self {
        Self::default()
    }

    /// Añade una pista al final de la lista. Si ya hay [`MAX_TRACKS`] pistas, la llamada se ignora.
    pub fn with_track(mut self, track: AxisTrack) -> Self {
        if let Some(slot) = self.tracks.iter_mut().find(|t| t.is_none()) {
            *slot = Some(track);
        }
        self
    }

    /// Repite la misma pista `count` veces (equivalente a `repeat(count, track)` en CSS, para el
    /// caso de uso más común con pistas idénticas). Trunca a [`MAX_TRACKS`] si `count` lo supera.
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let tracks = grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0));
    /// assert_eq!(tracks.to_string(), "1fr 1fr 1fr");
    /// ```
    pub fn repeat(count: usize, track: AxisTrack) -> Self {
        let mut tracks = Self::new();
        for _ in 0..count.min(MAX_TRACKS) {
            tracks = tracks.with_track(track);
        }
        tracks
    }

    // Serializa como valor CSS. Las pistas separadas por un espacio, en orden, ignorando los huecos
    // vacíos. Cadena vacía si no hay ninguna pista (para que `apply!` no genere ninguna clase,
    // igual que el resto de propiedades con valor "por defecto" de Flex).
    pub(crate) fn value(self) -> CowStr {
        self.tracks
            .iter()
            .flatten()
            .map(|t| t.value())
            .collect::<Vec<_>>()
            .join(" ")
            .into()
    }
}

// Único acceso público al valor CSS de `pub(crate) value()`, invisible fuera del crate, incluidos
// los propios doctests, que compilan como si fueran un crate externo (ver el ejemplo de `repeat()`,
// que depende de esta implementación para poder comprobar el resultado).
impl std::fmt::Display for Tracks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
