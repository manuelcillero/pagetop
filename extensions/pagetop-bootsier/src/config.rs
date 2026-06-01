//! Opciones de configuración del tema.
//!
//! Ejemplo:
//!
//! ```toml
//! [bootsier]
//! max_width = "90rem"
//! ```
//!
//! Uso:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! use pagetop_bootsier::config;
//!
//! assert_eq!(config::SETTINGS.bootsier.max_width, UnitValue::Px(1440));
//! ```
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [bootsier]
    "bootsier.max_width" => "1440px",
]);

/// Ajustes para la sección [`Bootsier`] de [`SETTINGS`].
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bootsier: Bootsier,
}

/// Sección **`[bootsier]`** de la configuración. Forma parte de [`Settings`].
#[derive(Debug, Deserialize)]
pub struct Bootsier {
    /// Ancho máximo predeterminado para la página, por ejemplo "100%" o "90rem".
    pub max_width: UnitValue,
}
