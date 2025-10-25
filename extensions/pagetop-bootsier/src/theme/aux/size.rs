use pagetop::prelude::*;

use std::fmt;

// **< ButtonSize >*********************************************************************************

/// Tamaño visual de un botón.
///
/// Controla la escala del botón según el diseño del tema:
///
/// - `Default`, tamaño por defecto del tema (no añade clase).
/// - `Small`, botón compacto.
/// - `Large`, botón destacado/grande.
#[derive(AutoDefault)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    Large,
}

#[rustfmt::skip]
impl fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Small   => f.write_str("btn-sm"),
            Self::Large   => f.write_str("btn-lg"),
        }
    }
}
