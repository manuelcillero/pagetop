//! Ajustes de configuración de `pagetop-catalog`.
//!
//! Ejemplo:
//!
//! ```toml
//! [catalog]
//! list_page_size = 50
//! csv_separator  = ";"
//! ```
//!
//! Uso:
//!
//! ```rust,no_run
//! use pagetop_catalog::config;
//!
//! assert_eq!(config::SETTINGS.list_page_size, 25);
//! ```
//!
//! Todas las opciones, con su valor por defecto, se documentan en [`CatalogSettings`].
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

use serde::Deserialize;

/// Ajustes de la sección `[catalog]`, cargados al arrancar la aplicación.
pub static SETTINGS: LazySettings<CatalogSettings> = LazySettings::new();

/// Sección **`[catalog]`** de la configuración.
#[settings(section = "catalog")]
pub struct CatalogSettings {
    /// Registros por página en el listado de cada catálogo.
    #[default = 25]
    pub list_page_size: u64,
    /// Separador de los campos de los CSV que se exportan: `","` o `";"`, el que espera Excel con
    /// la configuración regional de muchos países (España, Francia, Alemania...). Al importar, el
    /// separador se detecta en la cabecera del archivo.
    pub csv_separator: CsvSeparator,
    /// Filas que admite como mucho un CSV importado de una vez.
    #[default = 5000]
    pub import_max_rows: usize,
    /// Tamaño máximo en bytes de un CSV importado (por defecto 4 MiB), al subirlo y al confirmarlo:
    /// se lee entero en memoria. Es independiente de
    /// [`max_body_size`](pagetop::global::ServerSettings::max_body_size), que limita todos los
    /// formularios, también los públicos, porque importar exige el permiso de gestión del
    /// catálogo. Con *0* no tiene límite propio.
    ///
    /// No debe superar [`max_request_size`](pagetop::global::UploadSettings::max_request_size): un
    /// envío mayor lo rechaza antes el servidor, con `413 Payload Too Large` en lugar del mensaje
    /// del catálogo. Si lo supera, la aplicación lo avisa al arrancar.
    #[default = 4_194_304]
    pub import_max_size: usize,
}

// Aviso si un CSV que admite `import_max_size` lo rechazaría antes el servidor por superar
// `max_request_size` (*0* es sin límite en los dos).
pub(crate) fn import_size_warning(
    import_max_size: usize,
    max_request_size: usize,
) -> Option<String> {
    (max_request_size > 0 && (import_max_size == 0 || import_max_size > max_request_size)).then(
        || {
            format!(
                "catalog.import_max_size ({import_max_size}, 0 = no limit) allows more than \
                 upload.max_request_size ({max_request_size}): a larger CSV is rejected with 413"
            )
        },
    )
}

/// Separador de los campos de un CSV. Usado en [`CatalogSettings::csv_separator`].
#[derive(AutoDefault, Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum CsvSeparator {
    /// Coma.
    #[default]
    #[serde(rename = ",")]
    Comma,
    /// Punto y coma.
    #[serde(rename = ";")]
    Semicolon,
}

impl CsvSeparator {
    /// El carácter separador.
    pub const fn as_char(&self) -> char {
        match self {
            Self::Comma => ',',
            Self::Semicolon => ';',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CatalogSettings, config};

    // `reference.toml` documents every option; with unknown keys rejected, loading it proves that
    // it only documents keys that exist, with values of the right type.
    const REFERENCE: &str = include_str!("../../../config/reference.toml");

    #[test]
    fn reference_toml_documents_only_existing_keys() {
        config::load_from_str::<CatalogSettings>(REFERENCE).unwrap();
    }

    // The default sizes are consistent; a limit above the server's one, or none, is warned about.
    #[test]
    fn an_import_limit_above_the_request_limit_is_warned() {
        let request = pagetop::global::UploadSettings::default().max_request_size;
        let import = CatalogSettings::default().import_max_size;
        assert_eq!(super::import_size_warning(import, request), None);
        assert!(super::import_size_warning(request + 1, request).is_some());
        assert!(super::import_size_warning(0, request).is_some());
        assert_eq!(super::import_size_warning(request + 1, 0), None);
    }
}
