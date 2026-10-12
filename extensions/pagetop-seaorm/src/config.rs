//! Ajustes de configuración de `pagetop-seaorm`.
//!
//! Ejemplo:
//!
//! ```toml
//! [database]
//! db_type = "postgres"
//! db_name = "db"
//! db_user = "user"
//! db_pass = "password"
//! db_host = "localhost"
//! db_port = 5432
//! max_pool_size = 5
//! ```
//!
//! Uso:
//!
//! ```rust
//! use pagetop_seaorm::config;
//!
//! assert_eq!(config::SETTINGS.db_host, "localhost");
//! ```
//!
//! Todas las opciones, con su valor por defecto, se documentan en [`DatabaseSettings`].
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

use serde::Deserialize;

/// Ajustes de la sección `[database]`, cargados al arrancar la aplicación.
pub static SETTINGS: LazySettings<DatabaseSettings> = LazySettings::new();

/// Sección **`[database]`** de la configuración.
#[settings(section = "database")]
pub struct DatabaseSettings {
    /// Motor de base de datos.
    ///
    /// Valores aceptados: `"mysql"` (también `"mariadb"`), `"postgres"` (también `"postgresql"`) y
    /// `"sqlite"`. Si se omite, la aplicación terminará con un error al arrancar.
    pub db_type: DbType,
    /// Nombre (para mysql/postgres) o referencia (para sqlite) de la base de datos.
    pub db_name: String,
    /// Usuario de conexión a la base de datos (para mysql/postgres).
    pub db_user: String,
    /// Contraseña para la conexión a la base de datos (para mysql/postgres).
    pub db_pass: String,
    /// Servidor de conexión a la base de datos (para mysql/postgres).
    #[default = "localhost"]
    pub db_host: String,
    /// Puerto de conexión a la base de datos (para mysql/postgres). Si se omite, se usa el puerto
    /// predeterminado para el motor: 3306 para MySQL y 5432 para PostgreSQL.
    pub db_port: Option<u16>,
    /// Número máximo de conexiones habilitadas.
    #[default = 5]
    pub max_pool_size: u32,
}

/// Motor de base de datos. Usado en [`DatabaseSettings::db_type`].
#[derive(AutoDefault, Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DbType {
    /// Valor por defecto cuando `db_type` no está configurado. En este caso la aplicación terminará
    /// con un error al arrancar.
    #[default]
    #[serde(rename = "")]
    Unset,
    /// Usa el motor MySQL. Acepta también el alias `"mariadb"`.
    #[serde(alias = "mariadb")]
    Mysql,
    /// Usa el motor PostgreSQL. Acepta también el alias `"postgresql"`.
    #[serde(alias = "postgresql")]
    Postgres,
    /// Usa el motor SQLite.
    Sqlite,
}

#[cfg(test)]
mod tests {
    use super::{DatabaseSettings, config};

    // `reference.toml` documents every option; with unknown keys rejected, loading it proves that
    // it only documents keys that exist, with values of the right type.
    const REFERENCE: &str = include_str!("../../../config/reference.toml");

    #[test]
    fn reference_toml_documents_only_existing_keys() {
        config::load_from_str::<DatabaseSettings>(REFERENCE).unwrap();
    }
}
