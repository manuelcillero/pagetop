//! Es el tema básico que incluye `PageTop` por defecto.

use crate::prelude::*;

/// Tema básico por defecto.
pub struct Basic;

impl ExtensionTrait for Basic {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }
}

impl ThemeTrait for Basic {}
