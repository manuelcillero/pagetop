//! Opciones de configuración de la extensión.
//!
//! Ejemplo:
//!
//! ```toml
//! [database]
//! db_type = "mysql"
//! db_name = "db"
//! db_user = "user"
//! db_pass = "password"
//! db_host = "localhost"
//! db_port = 3306
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
    "database.db_type"       => "",
    "database.db_name"       => "",
    "database.db_user"       => "",
    "database.db_pass"       => "",
    "database.db_host"       => "localhost",
    "database.db_port"       => 0,
    "database.max_pool_size" => 5,
]);

#[derive(Debug, Deserialize)]
/// Tipos para la sección [`[database]`](Database) de [`SETTINGS`].
pub struct Settings {
    pub database: Database,
}

#[derive(Debug, Deserialize)]
/// Sección `[database]` de la configuración. Forma parte de [`Settings`].
pub struct Database {
    /// Tipo de base de datos: *"mysql"*, *"postgres"* ó *"sqlite"*.
    pub db_type: String,
    /// Nombre (para mysql/postgres) o referencia (para sqlite) de la base de datos.
    pub db_name: String,
    /// Usuario de conexión a la base de datos (para mysql/postgres).
    pub db_user: String,
    /// Contraseña para la conexión a la base de datos (para mysql/postgres).
    pub db_pass: String,
    /// Servidor de conexión a la base de datos (para mysql/postgres).
    pub db_host: String,
    /// Puerto de conexión a la base de datos, normalmente 3306 (para mysql) ó 5432 (para postgres).
    pub db_port: u16,
    /// Número máximo de conexiones habilitadas.
    pub max_pool_size: u32,
}
