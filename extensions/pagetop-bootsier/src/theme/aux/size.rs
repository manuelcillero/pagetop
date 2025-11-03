use pagetop::prelude::*;

use std::fmt;

// **< ButtonSize >*********************************************************************************

/// Tamaño visual de un botón.
#[derive(AutoDefault)]
pub enum ButtonSize {
    /// Tamaño por defecto del tema (no añade clase).
    #[default]
    Default,
    /// Botón compacto.
    Small,
    /// Botón destacado/grande.
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
