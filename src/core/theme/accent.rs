use crate::AutoDefault;

/// **Color de acento** de un componente visual.
///
/// A diferencia de [`Intent`](super::Intent), no tiene significado: sólo sirve para distinguir a la
/// vista unos elementos de otros (las secciones de una portada, las categorías de unas
/// etiquetas...).
///
/// Todos los componentes que lo admiten añaden la misma clase, `accent-<color>` (`accent-blue`,
/// `accent-orange`...; ver [`as_str()`](Self::as_str)), como hace
/// [`Tile`](crate::base::component::Tile). Cada tema define esas clases una sola vez: fijan una
/// variable CSS con el tono que el tema elige para ese color, y el estilo de cada componente sólo
/// lee esa variable. Basic usa `--val-color--accent` y Bootsier, `--bootsier-accent`.
///
/// Como cualquier variable CSS, se hereda: un elemento sin acento propio dentro de otro que lo
/// tiene toma el del contenedor. Por eso conviene que cada componente que admita un acento lo lleve
/// siempre (como `Tile`, azul por defecto) o lea la variable con un valor de reserva.
#[derive(AutoDefault, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Accent {
    /// Azul (por defecto).
    #[default]
    Blue,
    /// Añil.
    Indigo,
    /// Morado.
    Purple,
    /// Magenta.
    Magenta,
    /// Rosa.
    Pink,
    /// Rojo.
    Red,
    /// Naranja.
    Orange,
    /// Amarillo.
    Yellow,
    /// Lima.
    Lime,
    /// Verde.
    Green,
    /// Verde azulado.
    Teal,
    /// Cian.
    Cyan,
    /// Gris.
    Gray,
}

impl Accent {
    /// Devuelve el nombre del color (`"blue"`, `"orange"`...), para componer las clases CSS de cada
    /// componente.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Indigo => "indigo",
            Self::Purple => "purple",
            Self::Magenta => "magenta",
            Self::Pink => "pink",
            Self::Red => "red",
            Self::Orange => "orange",
            Self::Yellow => "yellow",
            Self::Lime => "lime",
            Self::Green => "green",
            Self::Teal => "teal",
            Self::Cyan => "cyan",
            Self::Gray => "gray",
        }
    }
}
