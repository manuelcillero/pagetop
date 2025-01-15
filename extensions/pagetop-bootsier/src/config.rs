//! Opciones de configuración.
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
//! ```rust#ignore
//! use pagetop_bootsier::config;
//!
//! assert_eq!(config::SETTINGS.bootsier.max_width, unit::Value::Rem(90));
//! ```
//!
//! Consulta [`pagetop::config`] para aprender cómo `PageTop` lee los archivos de opciones y aplica
//! los valores de configuración.

use pagetop::prelude::*;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [bootsier]
    "bootsier.max_width" => "1440px",
]);

#[derive(Debug, Deserialize)]
/// Opciones de configuración para la sección [`[bootsier]`](Bootsier) (ver [`SETTINGS`]).
pub struct Settings {
    pub bootsier: Bootsier,
}
#[derive(Debug, Deserialize)]
/// Sección `[bootsier]` de la configuración.
///
/// Ver [`Settings`].
pub struct Bootsier {
    /// Ancho máximo predeterminado para la página, por ejemplo "100%" o "90rem".
    /// Valor por defecto: *"1440px"*
    pub max_width: unit::Value,
}
