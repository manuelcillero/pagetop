//! Opciones de configuración.
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
//! See [`pagetop::include_config`] to learn how `PageTop` read configuration files and use
//! settings.

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
/// Represents configuration settings, specifically the [`[database]`](Database) section (used by
/// [`SETTINGS`]).
pub struct Settings {
    pub database: Database,
}
#[derive(Debug, Deserialize)]
/// Represents the `[database]` section in the [`Settings`] type.
pub struct Database {
    /// Type of database: *"mysql"*, *"postgres"*, or *"sqlite"*.
    /// Default: *""*.
    pub db_type: String,
    /// Name (for MySQL/Postgres) or reference (for SQLite) of the database.
    /// Default: *""*.
    pub db_name: String,
    /// Username for database connection (for MySQL/Postgres).
    /// Default: *""*.
    pub db_user: String,
    /// Password for database connection (for MySQL/Postgres).
    /// Default: *""*.
    pub db_pass: String,
    /// Hostname for database connection (for MySQL/Postgres).
    /// Default: *"localhost"*.
    pub db_host: String,
    /// Port number for database connection, typically 3306 (MySQL) or 5432 (Postgres).
    /// Default: *0*.
    pub db_port: u16,
    /// Maximum number of allowed connections.
    /// Default: *5*.
    pub max_pool_size: u32,
}
