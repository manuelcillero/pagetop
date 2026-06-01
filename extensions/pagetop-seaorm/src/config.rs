//! Opciones de configuración de la extensión.
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
//! # use pagetop_seaorm::config;
//! assert_eq!(config::SETTINGS.database.db_host, "localhost");
//! ```
//!
//! Consulta [`pagetop::config`] para ver cómo PageTop lee los archivos de configuración y aplica
//! los valores a los ajustes.

use pagetop::prelude::*;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [database]
    "database.db_type" => "",
    "database.db_name" => "",
    "database.db_user" => "",
    "database.db_pass" => "",
    "database.db_host" => "localhost",
    "database.max_pool_size" => 5,
]);

/// Ajustes para la sección [`Database`] de [`SETTINGS`].
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
}

/// Sección **`[database]`** de la configuración. Forma parte de [`Settings`].
#[derive(Debug, Deserialize)]
pub struct Database {
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
    pub db_host: String,
    /// Puerto de conexión a la base de datos (para mysql/postgres). Si se omite, se usa el puerto
    /// predeterminado para el motor: 3306 para MySQL y 5432 para PostgreSQL.
    pub db_port: Option<u16>,
    /// Número máximo de conexiones habilitadas.
    pub max_pool_size: u32,
}

/// Motor de base de datos. Usado en el campo [`Database::db_type`] de [`SETTINGS`].
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DbType {
    /// Valor por defecto cuando `db_type` no está configurado. En este caso la aplicación terminará
    /// con un error al arrancar.
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
