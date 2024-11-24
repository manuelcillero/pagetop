//! Configuration settings for SeaORM PageTop package.
//!
//! Example:
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
//! Usage:
//!
//! ```rust
//! use pagetop_seaorm::config;
//!
//! assert_eq!(config::SETTINGS.database.db_host, "localhost");
//! ```
//! See [`pagetop::config`](pagetop::config) to learn how **PageTop** read configuration files and
//! use settings.

use pagetop::prelude::*;

use serde::Deserialize;

include_config!(SETTINGS: Settings => [
    // [database]
    "database.db_type" => "",
    "database.db_name" => "",
    "database.db_user" => "",
    "database.db_pass" => "",
    "database.db_host" => "localhost",
    "database.db_port" => 0,
    "database.max_pool_size" => 5,
]);

#[derive(Debug, Deserialize)]
/// Type for HighlightJS configuration settings, section [`[hljs]`](Hljs) (used by [`SETTINGS`]).
pub struct Settings {
    pub database: Database,
}
#[derive(Debug, Deserialize)]
/// Struct for section `[database]` of [`Settings`] type.
pub struct Database {
    /// Tipo de base de datos: *"mysql"*, *"postgres"* ó *"sqlite"*.
    /// Por defecto: *""*.
    pub db_type: String,
    /// Nombre (para mysql/postgres) o referencia (para sqlite) de la base de datos.
    /// Por defecto: *""*.
    pub db_name: String,
    /// Usuario de conexión a la base de datos (para mysql/postgres).
    /// Por defecto: *""*.
    pub db_user: String,
    /// Contraseña para la conexión a la base de datos (para mysql/postgres).
    /// Por defecto: *""*.
    pub db_pass: String,
    /// Servidor de conexión a la base de datos (para mysql/postgres).
    /// Por defecto: *"localhost"*.
    pub db_host: String,
    /// Puerto de conexión a la base de datos, normalmente 3306 (para mysql) ó 5432 (para postgres).
    /// Por defecto: *0*.
    pub db_port: u16,
    /// Número máximo de conexiones habilitadas.
    /// Por defecto: *5*.
    pub max_pool_size: u32,
}
