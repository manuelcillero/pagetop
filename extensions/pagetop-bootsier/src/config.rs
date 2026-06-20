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
//! ```rust,no_run
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
    // [dev]
    "dev.bootsier_static_dir" => "",
]);

/// Ajustes para la sección [`Bootsier`] de [`SETTINGS`].
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bootsier: Bootsier,
    pub dev: Dev,
}

/// Sección **`[bootsier]`** de la configuración. Forma parte de [`Settings`].
#[derive(Debug, Deserialize)]
pub struct Bootsier {
    /// Ancho máximo predeterminado para la página, por ejemplo "100%" o "90rem".
    pub max_width: UnitValue,
}

/// Sección **`[dev]`** de la configuración. Forma parte de [`Settings`].
#[derive(Debug, Deserialize)]
pub struct Dev {
    /// Directorio raíz de `static/` para servir los archivos estáticos propios de Bootsier.
    ///
    /// Si se indica una ruta válida, absoluta o relativa al directorio del proyecto o del binario
    /// en ejecución, los archivos estáticos se servirán desde disco. Útil para poder modificar los
    /// archivos estáticos mientras la aplicación está en ejecución, sin necesidad de recompilar.
    ///
    /// Si la cadena está vacía, se ignora este ajuste.
    pub bootsier_static_dir: String,
}
