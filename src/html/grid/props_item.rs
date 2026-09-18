use crate::{AutoDefault, CowStr, util};

// **< ItemPlacement >******************************************************************************

/// Posición de un [`GridItem`](super::GridItem) en una línea de la rejilla (columna o fila).
///
/// El mismo tipo sirve para `grid-column` y `grid-row` ([`GridItem`](super::GridItem) guarda una
/// instancia independiente para cada eje).
///
/// Las líneas se numeran desde `1` (la primera línea antes de la primera pista); un número negativo
/// cuenta desde el final de la rejilla explícita (`-1` es la última línea). El `0` no es una línea
/// válida en CSS Grid y se trata como [`ItemPlacement::Auto`].
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemPlacement {
    /// Colocación automática, según `grid-auto-flow` del contenedor (valor por defecto, sin
    /// declaración explícita).
    #[default]
    Auto,
    /// Ocupa `n` pistas a partir de donde lo coloque el algoritmo de colocación automática
    /// (`span n`).
    Span(u16),
    /// Empieza en la línea indicada, ocupando una sola pista.
    Line(i16),
    /// Ocupa desde la primera línea hasta la segunda, ambas indicadas explícitamente
    /// (`<start> / <end>`).
    Range(i16, i16),
}

impl ItemPlacement {
    // Devuelve el valor CSS de `grid-column`/`grid-row`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Auto => "".into(),
            Self::Span(0) => "".into(),
            Self::Span(n) => util::join!("span ", n.to_string()).into(),
            Self::Line(0) => "".into(),
            Self::Line(n) => n.to_string().into(),
            Self::Range(0, _) | Self::Range(_, 0) => "".into(),
            Self::Range(start, end) => {
                util::join!(start.to_string(), " / ", end.to_string()).into()
            }
        }
    }
}

// **< ItemJustify >*********************************************************************************

/// Alineación individual en el eje de columnas de un [`GridItem`](super::GridItem).
///
/// Análogo a [`DefaultJustify`], pero para un único elemento (`justify-self`) en vez de para todos
/// los elementos del contenedor. Es la misma relación que ya tienen [`align::Items`] y
/// [`align::ItemSelf`] en [`crate::html::align`].
///
/// [`DefaultJustify`]: super::DefaultJustify
/// [`align::Items`]: crate::html::align::Items
/// [`align::ItemSelf`]: crate::html::align::ItemSelf
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum ItemJustify {
    /// Por defecto, hereda la alineación del contenedor (`justify-self: auto` no explícito).
    #[default]
    Default,
    /// Alinea el ítem al inicio de su columna (`justify-self: start`).
    Start,
    /// Alinea el ítem al final de su columna (`justify-self: end`).
    End,
    /// Centra el ítem en su columna (`justify-self: center`).
    Center,
    /// Estira el ítem para ocupar toda su columna (`justify-self: stretch`).
    Stretch,
}

impl ItemJustify {
    // Devuelve el valor CSS de `justify-self`, o "" para el valor por defecto.
    pub(super) fn value(self) -> CowStr {
        match self {
            Self::Default => "".into(),
            Self::Start => "start".into(),
            Self::End => "end".into(),
            Self::Center => "center".into(),
            Self::Stretch => "stretch".into(),
        }
    }
}
