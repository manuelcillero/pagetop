//! Ajustes de configuración de `pagetop-bootsier`.
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
//! assert_eq!(config::SETTINGS.max_width, UnitValue::RelRem(90.0));
//! ```
//!
//! Todas las opciones, con su valor por defecto, se documentan en [`BootsierSettings`].
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

/// Ajustes de la sección `[bootsier]`, cargados al arrancar la aplicación.
pub static SETTINGS: LazySettings<BootsierSettings> = LazySettings::new();

/// Sección **`[bootsier]`** de la configuración.
#[settings(section = "bootsier")]
pub struct BootsierSettings {
    /// Ancho máximo predeterminado para la página, por ejemplo "100%" o "90rem".
    #[default(UnitValue::Px(1440))]
    pub max_width: UnitValue,
    /// Directorio raíz de `static/` para servir los archivos estáticos propios de Bootsier.
    ///
    /// Si se indica una ruta válida, absoluta o relativa al directorio del proyecto (o, al ejecutar
    /// el binario, al directorio de trabajo actual), los archivos estáticos se servirán desde
    /// disco. Útil durante el desarrollo para poder modificar los archivos estáticos mientras la
    /// aplicación está en ejecución, sin necesidad de recompilar.
    ///
    /// Si la cadena está vacía, se ignora este ajuste.
    pub static_dir: String,
}

#[cfg(test)]
mod tests {
    use super::{BootsierSettings, config};

    // `reference.toml` documents every option; with unknown keys rejected, loading it proves that
    // it only documents keys that exist, with values of the right type.
    const REFERENCE: &str = include_str!("../../../config/reference.toml");

    #[test]
    fn reference_toml_documents_only_existing_keys() {
        config::load_from_str::<BootsierSettings>(REFERENCE).unwrap();
    }
}
